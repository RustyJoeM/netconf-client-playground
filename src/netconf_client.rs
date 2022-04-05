use anyhow::Result;
use quick_xml::{de::from_str, se::to_string};

use crate::{
    netconf::messages::{
        close_session::{CloseSessionRequest, CloseSessionResponse},
        hello::{HelloRequest, HelloResponse},
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

        let request = CloseSessionRequest::new(self.message_id);
        let request_str = to_string(&request)?;
        let response_str = self.ssh.dispatch_xml_request(&request_str)?;

        self.ssh.drop_channel();
        dbg!("channel dropped");
        let response = from_str(&response_str)?;
        Ok(response)
    }

    // pub fn connect(&mut self) -> Result<HelloServer> {
    //     self.ssh.connect()?;
    //     let reply: HelloServer = quick_xml::de::from_str(&self.get_reply()?).unwrap();
    //     if reply.is_ok() {
    //         self.session_id = Some(reply.session_id);
    //     }
    //     NetconfClient::make_return(reply)
    // }
}
