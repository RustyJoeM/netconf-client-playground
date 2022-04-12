use anyhow::Result;
use quick_xml::{de::from_str, se::to_string};

use crate::{
    netconf::{
        messages::{
            close_session::{CloseSessionRequest, CloseSessionResponse},
            get::{GetRequest, GetResponse},
            get_config::{GetConfigRequest, GetConfigResponse},
            hello::{HelloRequest, HelloResponse},
            kill_session::{KillSessionRequest, KillSessionResponse},
            lock::{LockRequest, LockResponse},
            unlock::{UnlockRequest, UnlockResponse},
        },
        types::{Datastore, Filter},
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

    fn new_message_id(&mut self) -> String {
        self.message_id = self.message_id.saturating_add(1);
        self.message_id.to_string()
    }

    pub fn connect(&mut self) -> Result<()> {
        self.ssh.connect()?;
        Ok(())
    }

    pub fn hello(&mut self) -> Result<HelloResponse> {
        let request = HelloRequest::new(vec![]);
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response: HelloResponse = from_str(&response_str)?;

        self.session_id = Some(response.session_id);

        Ok(response)
    }

    pub fn close_session(&mut self) -> Result<CloseSessionResponse> {
        let request = CloseSessionRequest::new(self.new_message_id());
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;

        self.ssh.drop_channel();

        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn lock(&mut self, datastore: Datastore) -> Result<LockResponse> {
        let request = LockRequest::new(self.new_message_id(), datastore);
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;

        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn unlock(&mut self, datastore: Datastore) -> Result<UnlockResponse> {
        let request = UnlockRequest::new(self.new_message_id(), datastore);
        let request_str = to_string(&request)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = from_str(&response_str)?;
        Ok(response)
    }

    pub fn get(&mut self, filter: Option<Filter>) -> Result<GetResponse> {
        let request_str = GetRequest::new_request_str(self.new_message_id(), filter)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = GetResponse::from_str(response_str)?;
        Ok(response)
    }

    pub fn get_config(
        &mut self,
        source: Datastore,
        filter: Option<Filter>,
    ) -> Result<GetConfigResponse> {
        let request_str = GetConfigRequest::new_request_str(self.new_message_id(), source, filter)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = GetConfigResponse::from_str(response_str)?;
        Ok(response)
    }

    pub fn kill_session(&mut self, session_id: u32) -> Result<KillSessionResponse> {
        let request = KillSessionRequest::new(self.new_message_id(), session_id);
        let request_str = to_string(&request)?;

        let response_str = self.ssh.dispatch_xml_request(&request_str)?;
        let response = from_str(&response_str)?;
        Ok(response)
    }
}
