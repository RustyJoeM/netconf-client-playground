mod common;
mod messages;
pub mod types;

use std::net::IpAddr;

// TODO - split to pub responses, or possibly pub requests as well?
use messages::{
    CloseSessionRequest, CloseSessionResponse, CopyConfigRequest, CopyConfigResponse,
    DeleteConfigRequest, DeleteConfigResponse, GetConfigRequest, GetConfigResponse, GetRequest,
    GetResponse, HelloRequest, HelloResponse, KillSessionRequest, KillSessionResponse, LockRequest,
    LockResponse, UnlockRequest, UnlockResponse,
};

mod ssh_client;
pub use ssh_client::SshAuthentication;
use ssh_client::SshClient;

use anyhow::Result;
use quick_xml::{de::from_str, se::to_string};

use crate::netconf_client::types::{Capability, Datastore, Filter, RpcReply};

/// The core NETCONF client. Simple blocking implementation that allows streaming
/// NETCONF RPC messages to a target server.
pub struct NetconfClient {
    ssh: SshClient,
    last_message_id: u32,
    session_id: Option<u32>,
    client_capabilities: Vec<Capability>,
    server_capabilities: Option<Vec<Capability>>,
}

impl NetconfClient {
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
        }
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

    /// Send <hello> request to target server. Client capabilities sent are the ones used at the creation of NETCONF server.
    /// These cannot be changed during session runtime.
    /// Server capabilities are stored in the [`Self`] instance after successful invocation.
    pub fn request_hello(&mut self) -> Result<HelloResponse> {
        let request = HelloRequest::new(self.client_capabilities.clone());
        let request_str = to_string(&request)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response: HelloResponse = from_str(&response_str)?;

        self.session_id = Some(response.session_id);
        self.server_capabilities = Some(response.capabilities.clone());

        Ok(response)
    }

    pub fn request_close_session(&mut self) -> Result<CloseSessionResponse> {
        let request = CloseSessionRequest::new(self.new_message_id());
        let request_str = to_string(&request)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response: CloseSessionResponse = from_str(&response_str)?;

        if RpcReply::Ok == response.reply {
            self.ssh.disconnect()?;
        }

        Ok(response)
    }

    pub fn request_lock(&mut self, datastore: Datastore) -> Result<LockResponse> {
        let request = LockRequest::new(self.new_message_id(), datastore);
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;

        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn request_unlock(&mut self, datastore: Datastore) -> Result<UnlockResponse> {
        let request = UnlockRequest::new(self.new_message_id(), datastore);
        let request_str = to_string(&request)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn request_get(&mut self, filter: Option<Filter>) -> Result<GetResponse> {
        let request_str = GetRequest::new_request_str(self.new_message_id(), filter)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = GetResponse::from_str(response_str)?;
        Ok(response)
    }

    pub fn request_get_config(
        &mut self,
        source: Datastore,
        filter: Option<Filter>,
    ) -> Result<GetConfigResponse> {
        let request_str = GetConfigRequest::new_request_str(self.new_message_id(), source, filter)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = GetConfigResponse::from_str(response_str)?;
        Ok(response)
    }

    pub fn request_edit_config(
        &mut self,
        target: Datastore,
        source: Datastore,
    ) -> Result<CopyConfigResponse> {
        let request = CopyConfigRequest::new(self.new_message_id(), target, source);
        let request_str = to_string(&request)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn request_copy_config(&mut self, datastore: Datastore) -> Result<UnlockResponse> {
        let request = UnlockRequest::new(self.new_message_id(), datastore);
        let request_str = to_string(&request)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn request_delete_config(&mut self, target: Datastore) -> Result<DeleteConfigResponse> {
        let request = DeleteConfigRequest::new(self.new_message_id(), target);
        let request_str = to_string(&request)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn request_kill_session(&mut self, session_id: u32) -> Result<KillSessionResponse> {
        let request = KillSessionRequest::new(self.new_message_id(), session_id);
        let request_str = to_string(&request)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = from_str(&response_str)?;
        Ok(response)
    }
}
