use std::net::IpAddr;

use super::messages::*;

use super::messages::edit_config::EditConfigContent;
use super::ssh_client::SshClient;
use super::types::{Capability, ConfigWaypoint, Datastore, Filter, RpcReply};
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
        self.ssh.connect()
    }

    /// Increase internal message-id counter and return its String representation.
    fn new_message_id(&mut self) -> String {
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

    /// Send <hello> request to target server. Client capabilities sent are the ones used at the creation of NETCONF server.
    /// These cannot be changed during session runtime.
    /// Server capabilities are stored in the [`Self`] instance after successful invocation.
    pub fn request_hello(&mut self) -> Result<hello::HelloResponse> {
        let request = hello::HelloRequest::new(self.client_capabilities.clone());
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        let response = hello::HelloResponse::from_netconf_rpc(response_str)?;

        self.session_id = Some(response.session_id);
        self.server_capabilities = Some(response.capabilities.clone());

        // RFC quote:
        // If no protocol version capability in common is found, the NETCONF peer MUST NOT continue the
        // session. If more than one protocol version URI in common is present, then the highest numbered
        // (most recent) protocol version MUST be used by both peers.
        if self.validate_capabilities {
            for base in vec![Capability::Base11, Capability::Base] {
                if !self.client_capabilities.contains(&base) {
                    continue;
                }
                if let Some(caps) = &self.server_capabilities {
                    if caps.contains(&base) {
                        return Ok(response);
                    }
                }
            }
            bail!("No common base capability found!".to_string())
        }

        Ok(response)
    }

    pub fn request_close_session(&mut self) -> Result<close_session::CloseSessionResponse> {
        let request = close_session::CloseSessionRequest::new(self.new_message_id());
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        let response = close_session::CloseSessionResponse::from_netconf_rpc(response_str)?;

        if RpcReply::Ok == response.reply {
            self.ssh.disconnect()?;
        }

        Ok(response)
    }

    pub fn request_lock(&mut self, datastore: Datastore) -> Result<lock::LockResponse> {
        let request = lock::LockRequest::new(self.new_message_id(), datastore);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        lock::LockResponse::from_netconf_rpc(response_str)
    }

    pub fn request_unlock(&mut self, datastore: Datastore) -> Result<unlock::UnlockResponse> {
        let request = unlock::UnlockRequest::new(self.new_message_id(), datastore);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        unlock::UnlockResponse::from_netconf_rpc(response_str)
    }

    pub fn request_get(&mut self, filter: Option<Filter>) -> Result<get::GetResponse> {
        let request = get::GetRequest::new(self.new_message_id(), filter);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        get::GetResponse::from_netconf_rpc(response_str)
    }

    pub fn request_get_config(
        &mut self,
        source: Datastore,
        filter: Option<Filter>,
    ) -> Result<get_config::GetConfigResponse> {
        let request = get_config::GetConfigRequest::new(self.new_message_id(), source, filter);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        get_config::GetConfigResponse::from_netconf_rpc(response_str)
    }

    // TODO - untested - possibly unfinished/incorrect (de)serialization...
    pub fn request_edit_config(
        &mut self,
        params: edit_config::EditConfigParams,
    ) -> Result<edit_config::EditConfigResponse> {
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
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        edit_config::EditConfigResponse::from_netconf_rpc(response_str)
    }

    pub fn request_copy_config(
        &mut self,
        target: ConfigWaypoint,
        source: ConfigWaypoint,
    ) -> Result<copy_config::CopyConfigResponse> {
        if self.validate_capabilities
            && target == ConfigWaypoint::Datastore(Datastore::Running)
            && !self.got_server_capability(Capability::WritableRunning)
        {
            bail!("Cannot write to running datastore, server didn't advertise :writable-running capability!");
        }

        let request = copy_config::CopyConfigRequest::new(self.new_message_id(), target, source);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        copy_config::CopyConfigResponse::from_netconf_rpc(response_str)
    }

    pub fn request_delete_config(
        &mut self,
        target: ConfigWaypoint,
    ) -> Result<delete_config::DeleteConfigResponse> {
        let request = delete_config::DeleteConfigRequest::new(self.new_message_id(), target);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        delete_config::DeleteConfigResponse::from_netconf_rpc(response_str)
    }

    pub fn request_kill_session(
        &mut self,
        session_id: u32,
    ) -> Result<kill_session::KillSessionResponse> {
        let request = kill_session::KillSessionRequest::new(self.new_message_id(), session_id);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        kill_session::KillSessionResponse::from_netconf_rpc(response_str)
    }

    fn commit(
        &mut self,
        params: Option<commit::ConfirmedCommitParams>,
    ) -> Result<commit::CommitResponse> {
        let request = commit::CommitRequest::new(self.new_message_id(), params);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        commit::CommitResponse::from_netconf_rpc(response_str)
    }

    pub fn request_commit(&mut self) -> Result<commit::CommitResponse> {
        if self.validate_capabilities && !self.got_server_capability(Capability::Candidate) {
            bail!("<commit> operation allowed only for :candidate enabled servers!");
        };
        self.commit(None)
    }

    pub fn request_confirmed_commit(
        &mut self,
        params: commit::ConfirmedCommitParams,
    ) -> Result<commit::CommitResponse> {
        if self.validate_capabilities && !self.got_server_capability(Capability::ConfirmedCommit) {
            bail!("<commit> operation with parameters allowed only for :confirmed-commit enabled servers!");
        };
        self.commit(Some(params))
    }

    pub fn request_discard_changes(&mut self) -> Result<discard_changes::DiscardChangesResponse> {
        if self.validate_capabilities && !self.got_server_capability(Capability::Candidate) {
            bail!("<discard-changes> operation allowed only for :candidate enabled servers!");
        };
        let request = discard_changes::DiscardChangesRequest::new(self.new_message_id());
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        discard_changes::DiscardChangesResponse::from_netconf_rpc(response_str)
    }

    pub fn request_cancel_commit(
        &mut self,
        persist_id: Option<u32>,
    ) -> Result<cancel_commit::CancelCommitResponse> {
        if self.validate_capabilities && !self.got_server_capability(Capability::ConfirmedCommit) {
            bail!("<cancel-commit> operation allowed only for :confirmed-commit enabled servers!");
        };
        let request = cancel_commit::CancelCommitRequest::new(self.new_message_id(), persist_id);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        cancel_commit::CancelCommitResponse::from_netconf_rpc(response_str)
    }

    // TODO check for :candidate in <get-config>, <edit-config>, <copy-config>, and <validate>
    // TODO check for :candidate in <lock>, <unlock>

    pub fn request_validate(
        &mut self,
        source: validate::ValidateSource,
    ) -> Result<validate::ValidateResponse> {
        // TODO - validate 1.0 vs 1.1 depending on client
        let request = validate::ValidateRequest::new(self.new_message_id(), source);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        validate::ValidateResponse::from_netconf_rpc(response_str)
    }
}
