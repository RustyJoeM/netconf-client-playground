use std::net::IpAddr;

// TODO - split to pub responses, or possibly pub requests as well?
use super::messages::{
    CancelCommitRequest, CancelCommitResponse, CloseSessionRequest, CloseSessionResponse,
    CommitRequest, CommitResponse, CopyConfigRequest, CopyConfigResponse, DeleteConfigRequest,
    DeleteConfigResponse, DiscardChangesRequest, DiscardChangesResponse, EditConfigRequest,
    EditConfigResponse, GetConfigRequest, GetConfigResponse, GetRequest, GetResponse, HelloRequest,
    HelloResponse, KillSessionRequest, KillSessionResponse, LockRequest, LockResponse,
    NetconfResponse, UnlockRequest, UnlockResponse,
};

pub use super::messages::{ConfirmedCommitParams, EditConfigParams};
use super::ssh_client::SshClient;
use super::types::{Capability, Datastore, Filter, RpcReply};
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
    pub fn request_hello(&mut self) -> Result<HelloResponse> {
        let request = HelloRequest::new(self.client_capabilities.clone());
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        let response = HelloResponse::from_netconf_rpc(response_str)?;

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

    pub fn request_close_session(&mut self) -> Result<CloseSessionResponse> {
        let request = CloseSessionRequest::new(self.new_message_id());
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        let response = CloseSessionResponse::from_netconf_rpc(response_str)?;

        if RpcReply::Ok == response.reply {
            self.ssh.disconnect()?;
        }

        Ok(response)
    }

    pub fn request_lock(&mut self, datastore: Datastore) -> Result<LockResponse> {
        let request = LockRequest::new(self.new_message_id(), datastore);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        LockResponse::from_netconf_rpc(response_str)
    }

    pub fn request_unlock(&mut self, datastore: Datastore) -> Result<UnlockResponse> {
        let request = UnlockRequest::new(self.new_message_id(), datastore);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        UnlockResponse::from_netconf_rpc(response_str)
    }

    pub fn request_get(&mut self, filter: Option<Filter>) -> Result<GetResponse> {
        let request = GetRequest::new(self.new_message_id(), filter);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        GetResponse::from_netconf_rpc(response_str)
    }

    pub fn request_get_config(
        &mut self,
        source: Datastore,
        filter: Option<Filter>,
    ) -> Result<GetConfigResponse> {
        let request = GetConfigRequest::new(self.new_message_id(), source, filter);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        GetConfigResponse::from_netconf_rpc(response_str)
    }

    // TODO - untested - possibly unfinished/incorrect (de)serialization...
    pub fn request_edit_config(&mut self, params: EditConfigParams) -> Result<EditConfigResponse> {
        if self.validate_capabilities
            && params.target == Datastore::Running
            && !self.got_server_capability(Capability::WritableRunning)
        {
            bail!("Cannot write to running datastore, server didn't advertise :writable-running capability!");
        };

        let request = EditConfigRequest::new(self.new_message_id(), params);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        EditConfigResponse::from_netconf_rpc(response_str)
    }

    pub fn request_copy_config(
        &mut self,
        target: Datastore,
        source: Datastore,
    ) -> Result<CopyConfigResponse> {
        if self.validate_capabilities
            && target == Datastore::Running
            && !self.got_server_capability(Capability::WritableRunning)
        {
            bail!("Cannot write to running datastore, server didn't advertise :writable-running capability!");
        };

        let request = CopyConfigRequest::new(self.new_message_id(), target, source);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        CopyConfigResponse::from_netconf_rpc(response_str)
    }

    pub fn request_delete_config(&mut self, target: Datastore) -> Result<DeleteConfigResponse> {
        let request = DeleteConfigRequest::new(self.new_message_id(), target);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        DeleteConfigResponse::from_netconf_rpc(response_str)
    }

    pub fn request_kill_session(&mut self, session_id: u32) -> Result<KillSessionResponse> {
        let request = KillSessionRequest::new(self.new_message_id(), session_id);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        KillSessionResponse::from_netconf_rpc(response_str)
    }

    fn commit(&mut self, params: Option<ConfirmedCommitParams>) -> Result<CommitResponse> {
        let request = CommitRequest::new(self.new_message_id(), params);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        CommitResponse::from_netconf_rpc(response_str)
    }

    pub fn request_commit(&mut self) -> Result<CommitResponse> {
        if self.validate_capabilities && !self.got_server_capability(Capability::Candidate) {
            bail!("<commit> operation allowed only for :candidate enabled servers!");
        };
        self.commit(None)
    }

    pub fn request_confirmed_commit(
        &mut self,
        params: ConfirmedCommitParams,
    ) -> Result<CommitResponse> {
        if self.validate_capabilities && !self.got_server_capability(Capability::ConfirmedCommit) {
            bail!("<commit> operation with parameters allowed only for :confirmed-commit enabled servers!");
        };
        self.commit(Some(params))
    }

    pub fn request_discard_changes(&mut self) -> Result<DiscardChangesResponse> {
        if self.validate_capabilities && !self.got_server_capability(Capability::Candidate) {
            bail!("<discard-changes> operation allowed only for :candidate enabled servers!");
        };
        let request = DiscardChangesRequest::new(self.new_message_id());
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        DiscardChangesResponse::from_netconf_rpc(response_str)
    }

    pub fn request_cancel_commit(
        &mut self,
        persist_id: Option<u32>,
    ) -> Result<CancelCommitResponse> {
        if self.validate_capabilities && !self.got_server_capability(Capability::ConfirmedCommit) {
            bail!("<cancel-commit> operation allowed only for :confirmed-commit enabled servers!");
        };
        let request = CancelCommitRequest::new(self.new_message_id(), persist_id);
        let response_str = self.ssh.dispatch_netconf_request(&request)?;
        CancelCommitResponse::from_netconf_rpc(response_str)
    }

    // TODO check for :candidate in <get-config>, <edit-config>, <copy-config>, and <validate>
    // TODO check for :candidate in <lock>, <unlock>
}
