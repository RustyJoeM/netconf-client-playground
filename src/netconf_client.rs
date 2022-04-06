use anyhow::Result;
use quick_xml::{de::from_str, se::to_string};

use crate::{
    netconf::{
        messages::{
            close_session::{CloseSessionRequest, CloseSessionResponse},
            hello::{HelloRequest, HelloResponse},
            kill_session::{KillSessionRequest, KillSessionResponse},
            lock::{LockRequest, LockResponse},
            unlock::{UnlockRequest, UnlockResponse},
        },
        types::Datastore,
    },
    ssh_client::{SshAuthentication, SshClient},
};

pub struct NetconfClient {
    ssh: SshClient,
    message_id: u32,
    pub session_id: Option<u32>,
}

impl NetconfClient {
    pub fn new(address: &str, port: u16, auth: SshAuthentication) -> Self {
        Self {
            ssh: SshClient::new(address, port, auth),
            message_id: 0,
            session_id: None,
        }
    }

    fn increase_message_id(&mut self) {
        self.message_id = self.message_id.saturating_add(1);
    }

    pub fn connect(&mut self) -> Result<()> {
        self.ssh.connect()?;
        Ok(())
    }

    pub fn hello(&mut self) -> Result<HelloResponse> {
        self.increase_message_id();

        let request = HelloRequest::new(vec![]);
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response: HelloResponse = from_str(&response_str)?;

        self.session_id = Some(response.session_id);

        Ok(response)
    }

    pub fn close_session(&mut self) -> Result<CloseSessionResponse> {
        self.increase_message_id();

        let request = CloseSessionRequest::new(self.message_id.to_string());
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;

        self.ssh.drop_channel();
        dbg!("channel dropped");
        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn lock(&mut self, datastore: Datastore) -> Result<LockResponse> {
        self.increase_message_id();

        let request = LockRequest::new(self.message_id.to_string(), datastore);
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;

        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn unlock(&mut self, datastore: Datastore) -> Result<UnlockResponse> {
        self.increase_message_id();

        let request = UnlockRequest::new(self.message_id.to_string(), datastore);
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;

        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn kill_session(&mut self, session_id: u32) -> Result<KillSessionResponse> {
        self.increase_message_id();

        let request = KillSessionRequest::new(self.message_id.to_string(), session_id);
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;

        let response = from_str(&response_str)?;
        Ok(response)
    }
}
