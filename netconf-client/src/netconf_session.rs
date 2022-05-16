use std::net::IpAddr;

use crate::messages::hello::HelloResponse;
use crate::ssh_client::BaseCapability;

use super::messages::*;

use super::messages::edit_config::EditConfigContent;
use super::ssh_client::SshClient;
use super::types::{Capability, ConfigWaypoint, Datastore, FilterPayload, RpcReply};
use super::SshAuthentication;

use anyhow::{bail, Result};

/// The core NETCONF client. Simple blocking implementation that allows streaming
/// NETCONF RPC messages to a target server.
pub struct NetconfSession {
    ssh: SshClient,
    last_message_id: u32,
    session_id: Option<u32>,
    client_capabilities: Vec<Capability>,
    server_capabilities: Option<Vec<Capability>>,
    validate_capabilities: bool,
}

impl NetconfSession {
    /// Create new instance of NETCONF client, without connecting to the server.
    /// Invoke [`Self::connect()`] to establish the connection, prior to other requests.
    pub fn new(
        address: IpAddr,
        port: u16,
        auth: SshAuthentication,
        client_capabilities: Vec<Capability>,
    ) -> Self {
        Self {
            ssh: SshClient::new(address, port, auth),
            last_message_id: 0,
            session_id: None,
            client_capabilities,
            server_capabilities: None,
            validate_capabilities: true,
        }
    }

    pub fn target_string(&self) -> String {
        self.ssh.target_string()
    }

    pub fn session_id(&self) -> Option<u32> {
        self.session_id
    }

    /// Set whether to perform server capabilities check before dispatching the actual RPCs to server.
    ///
    /// Upon initial \<hello\> exchange when session is created, capabilities advertised by NETCONF server are stored internally.
    /// All subsequent checks are executed "offline" against this capability set.
    ///
    /// This setting is `true` by default.
    pub fn set_validate_capabilities(&mut self, do_validate: bool) {
        self.validate_capabilities = do_validate;
    }

    /// Establish connection to target server.
    pub fn connect(&mut self) -> Result<()> {
        if self.session_id.is_some() {
            bail!("Already connected to server!");
        }
        self.ssh.connect()
    }

    /// Increase internal message-id counter and return its String representation.
    pub fn new_message_id(&mut self) -> String {
        self.last_message_id = self.last_message_id.saturating_add(1);
        self.last_message_id.to_string()
    }

    /// Check whether specific Capability is among the ones advertised by the connected NETCONF server.
    fn got_server_capability(&self, cap: Capability) -> bool {
        match &self.server_capabilities {
            Some(caps) => caps.contains(&cap),
            None => false,
        }
    }

    /// All-in-one constructor that connects to the target NETCONF server,
    /// and exchanges the \<hello\> messages and capabilities information.
    pub fn initialize(
        address: IpAddr,
        port: u16,
        auth: SshAuthentication,
        client_capabilities: Vec<Capability>,
    ) -> Result<Self> {
        let mut instance = Self::new(address, port, auth, client_capabilities);
        instance.connect()?;
        instance.request_hello()?;
        Ok(instance)
    }

    pub fn server_capabilities(&self) -> &Option<Vec<Capability>> {
        &self.server_capabilities
    }

    pub fn base_capability(&self) -> BaseCapability {
        self.ssh.base_capability()
    }

    pub fn dispatch_request<R: NetconfRequest>(
        &mut self,
        request: R,
    ) -> Result<FullResponse<R::Response>> {
        let dump = self.ssh.dispatch_netconf_request(&request)?;
        let typed = R::Response::from_netconf_rpc(&dump)?;
        Ok(FullResponse { typed, dump })
    }

    // TODO - address automated connection process not to enforce user to invoke multiple of fns in sequence...
    /// Update internal state of [`NetconfSession`] according to the hello capabilities exchange results.
    pub fn update_on_hello(&mut self, response: &HelloResponse) -> Result<()> {
        // RFC quote:
        // If no protocol version capability in common is found, the NETCONF peer MUST NOT continue the
        // session. If more than one protocol version URI in common is present, then the highest numbered
        // (most recent) protocol version MUST be used by both peers.
        let base_capability =
            common_highest_base_capability(&self.client_capabilities, &response.capabilities);

        match base_capability {
            Some(base_capability) => self.ssh.set_base_capability(base_capability),
            None => bail!("No common base capability found!".to_string()),
        };

        self.session_id = Some(response.session_id);
        self.server_capabilities = Some(response.capabilities.clone());

        Ok(())
    }

    /// Send <hello> request to target server. Client capabilities sent are the ones used at the creation of NETCONF server.
    /// These cannot be changed during session runtime.
    /// Server capabilities are stored in the [`Self`] instance after successful invocation.
    pub fn request_hello(&mut self) -> Result<FullResponse<hello::HelloResponse>> {
        let request = hello::HelloRequest::new(self.client_capabilities.clone());
        let response = self.dispatch_request(request)?;

        self.update_on_hello(&response.typed)?;

        Ok(response)
    }

    pub fn request_close_session(
        &mut self,
    ) -> Result<FullResponse<close_session::CloseSessionResponse>> {
        let request = close_session::CloseSessionRequest::new(self.new_message_id());
        let response = self.dispatch_request(request)?;

        if RpcReply::Ok == response.typed.reply {
            self.ssh.disconnect()?;
        }

        Ok(response)
    }

    pub fn request_lock(
        &mut self,
        datastore: Datastore,
    ) -> Result<FullResponse<lock::LockResponse>> {
        let request = lock::LockRequest::new(self.new_message_id(), datastore);
        self.dispatch_request(request)
    }

    pub fn request_unlock(
        &mut self,
        datastore: Datastore,
    ) -> Result<FullResponse<unlock::UnlockResponse>> {
        let request = unlock::UnlockRequest::new(self.new_message_id(), datastore);
        self.dispatch_request(request)
    }

    pub fn request_get(
        &mut self,
        filter: Option<FilterPayload>,
    ) -> Result<FullResponse<get::GetResponse>> {
        let request = get::GetRequest::new(self.new_message_id(), filter);
        self.dispatch_request(request)
    }

    pub fn request_get_config(
        &mut self,
        source: Datastore,
        filter: Option<FilterPayload>,
    ) -> Result<FullResponse<get_config::GetConfigResponse>> {
        let request = get_config::GetConfigRequest::new(self.new_message_id(), source, filter);
        self.dispatch_request(request)
    }

    // TODO - untested - possibly unfinished/incorrect (de)serialization...
    pub fn request_edit_config(
        &mut self,
        params: edit_config::EditConfigParams,
    ) -> Result<FullResponse<edit_config::EditConfigResponse>> {
        if self.validate_capabilities {
            if params.target == Datastore::Running
                && !self.got_server_capability(Capability::WritableRunning)
            {
                bail!("Cannot write to running datastore, server didn't advertise :writable-running capability!");
            };
            if let Some(error_option) = &params.error_option {
                if *error_option == edit_config::ErrorOption::RollbackOnError
                    && !self.got_server_capability(Capability::RollbackOnError)
                {
                    bail!("Server didn't advertise :rollback-on-error capability! Cannot use this error-option.");
                }
            }
            if params.test_option.is_some() && !self.got_server_capability(Capability::Validate11) {
                bail!("Server didn't advertise :validate:1.1 capability! Cannot use test-option.");
            }
            if let EditConfigContent::Url(_) = &params.config {
                // TODO - check scheme used -> getter for generic Url capability - check schemes supported by server vs the one being used by caller
            }
        }

        let request = edit_config::EditConfigRequest::new(self.new_message_id(), params);
        self.dispatch_request(request)
    }

    pub fn request_copy_config(
        &mut self,
        target: ConfigWaypoint,
        source: ConfigWaypoint,
    ) -> Result<FullResponse<copy_config::CopyConfigResponse>> {
        if self.validate_capabilities
            && target == ConfigWaypoint::Datastore(Datastore::Running)
            && !self.got_server_capability(Capability::WritableRunning)
        {
            bail!("Cannot write to running datastore, server didn't advertise :writable-running capability!");
        }

        let request = copy_config::CopyConfigRequest::new(self.new_message_id(), target, source);
        self.dispatch_request(request)
    }

    pub fn request_delete_config(
        &mut self,
        target: ConfigWaypoint,
    ) -> Result<FullResponse<delete_config::DeleteConfigResponse>> {
        let request = delete_config::DeleteConfigRequest::new(self.new_message_id(), target);
        self.dispatch_request(request)
    }

    pub fn request_kill_session(
        &mut self,
        session_id: u32,
    ) -> Result<FullResponse<kill_session::KillSessionResponse>> {
        let request = kill_session::KillSessionRequest::new(self.new_message_id(), session_id);
        self.dispatch_request(request)
    }

    fn commit(
        &mut self,
        params: Option<commit::ConfirmedCommitParams>,
    ) -> Result<FullResponse<commit::CommitResponse>> {
        let request = commit::CommitRequest::new(self.new_message_id(), params);
        self.dispatch_request(request)
    }

    pub fn request_commit(&mut self) -> Result<FullResponse<commit::CommitResponse>> {
        if self.validate_capabilities && !self.got_server_capability(Capability::Candidate) {
            bail!("<commit> operation allowed only for :candidate enabled servers!");
        };
        self.commit(None)
    }

    pub fn request_confirmed_commit(
        &mut self,
        params: commit::ConfirmedCommitParams,
    ) -> Result<FullResponse<commit::CommitResponse>> {
        if self.validate_capabilities && !self.got_server_capability(Capability::ConfirmedCommit) {
            bail!("<commit> operation with parameters allowed only for :confirmed-commit enabled servers!");
        };
        self.commit(Some(params))
    }

    pub fn request_discard_changes(
        &mut self,
    ) -> Result<FullResponse<discard_changes::DiscardChangesResponse>> {
        if self.validate_capabilities && !self.got_server_capability(Capability::Candidate) {
            bail!("<discard-changes> operation allowed only for :candidate enabled servers!");
        };
        let request = discard_changes::DiscardChangesRequest::new(self.new_message_id());
        self.dispatch_request(request)
    }

    pub fn request_cancel_commit(
        &mut self,
        persist_id: Option<u32>,
    ) -> Result<FullResponse<cancel_commit::CancelCommitResponse>> {
        if self.validate_capabilities && !self.got_server_capability(Capability::ConfirmedCommit) {
            bail!("<cancel-commit> operation allowed only for :confirmed-commit enabled servers!");
        };
        let request = cancel_commit::CancelCommitRequest::new(self.new_message_id(), persist_id);
        self.dispatch_request(request)
    }

    // TODO check for :candidate in <get-config>, <edit-config>, <copy-config>, and <validate>
    // TODO check for :candidate in <lock>, <unlock>

    pub fn request_validate(
        &mut self,
        source: validate::ValidateSource,
    ) -> Result<FullResponse<validate::ValidateResponse>> {
        // TODO - validate 1.0 vs 1.1 depending on client
        let request = validate::ValidateRequest::new(self.new_message_id(), source);
        self.dispatch_request(request)
    }
}

fn common_highest_base_capability(
    client_capabilities: &[Capability],
    server_capabilities: &[Capability],
) -> Option<BaseCapability> {
    let common_cap = [Capability::Base11, Capability::Base]
        .iter()
        .find(|base| client_capabilities.contains(base) && server_capabilities.contains(base));

    match common_cap {
        Some(cap) => match cap {
            Capability::Base => Some(BaseCapability::Base),
            Capability::Base11 => Some(BaseCapability::Base11),
            _ => None,
        },
        None => None,
    }
}
