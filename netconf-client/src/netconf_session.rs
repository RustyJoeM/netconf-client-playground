use std::net::IpAddr;

use crate::messages::hello::HelloResponse;
use crate::ssh_client::BaseCapability;

use super::messages::*;

use super::ssh_client::SshClient;
use super::types::{Capability, RpcReply};
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

    pub fn validate_capabilities(&self) -> bool {
        self.validate_capabilities
    }

    /// Set whether to perform server capabilities check before dispatching the actual RPCs to server.
    ///
    /// Upon initial `<hello>` exchange when session is created, capabilities advertised by NETCONF server are stored internally.
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

    /// All-in-one constructor that connects to the target NETCONF server,
    /// and exchanges the `<hello>` messages and capabilities information.
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

    pub fn server_capabilities(&self) -> Option<&[Capability]> {
        self.server_capabilities.as_deref()
    }

    pub fn base_capability(&self) -> BaseCapability {
        self.ssh.base_capability()
    }

    /// Core method for dispatching NETCONF requests to server.
    pub fn dispatch_request<R: NetconfRequest>(
        &mut self,
        request: R,
    ) -> Result<FullResponse<R::Response>> {
        if self.validate_capabilities {
            match self.server_capabilities() {
                Some(caps) => request.validate_request(caps)?,
                None => bail!("No server capabilities! Session not initiated yet?"),
            };
        }
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
