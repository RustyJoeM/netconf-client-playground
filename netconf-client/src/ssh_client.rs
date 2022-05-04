//! Simple blocking implementation of SSH client, that allows exchanging RPC
//! requests/responses with target NETCONF server.

use anyhow::{bail, Result};
use ssh2::{Channel, Session};

use std::{
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpStream},
};

use super::messages::NetconfRequest;

/// RFC specified NETCONF message separator.
const MESSAGE_SEPARATOR: &str = "]]>]]>";
const SSH_TIMEOUT: u32 = 5000;

/// Type of authentication used for SSH connection.
#[derive(Debug)]
pub enum SshAuthentication {
    /// Plain old username & password access. Please note plain-text data kept in memory during runtime.
    UserPassword(String, String),
}

/// SSH client for streaming messages between caller - client, and NETCONF server.
pub struct SshClient {
    address: IpAddr,
    port: u16,
    auth: SshAuthentication,
    channel: Option<Channel>,
}

impl SshClient {
    /// Creates new instance of SSH client, without initiating any network connection yet.
    /// Prior to any other sub-sequent operations, [`Self::connect()`] must be invoked.
    pub fn new(address: IpAddr, port: u16, auth: SshAuthentication) -> Self {
        Self {
            address,
            port,
            auth,
            channel: None,
        }
    }

    pub fn target_string(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    /// Connect to target NETCONF server - open the SSH session via TCP stream and authenticate.
    /// Must be invoked after creation of [`Self`] prior to dispatching any messages.
    pub fn connect(&mut self) -> Result<()> {
        let mut session = Session::new()?;
        session.set_blocking(true);
        session.set_timeout(SSH_TIMEOUT);
        let socket_address = SocketAddr::from((self.address, self.port));
        let tcp_stream = TcpStream::connect(socket_address)?;
        session.set_tcp_stream(tcp_stream);
        session.handshake()?;

        match &self.auth {
            SshAuthentication::UserPassword(username, password) => {
                session.userauth_password(username, password)?;
            }
        }

        let mut channel = session.channel_session()?;
        channel.subsystem("netconf")?;
        self.channel = Some(channel);

        Ok(())
    }

    /// Disconnects the instance of [`Self`] from connected NETCONF server.
    /// Instance can be re-used subsequently if needed, with another connect() invocation.
    pub fn disconnect(&mut self) -> Result<()> {
        let channel = self.channel.as_mut().unwrap();
        channel.send_eof()?;
        channel.wait_eof()?;
        match channel.eof() {
            true => {}
            false => return Err(ssh2::Error::eof().into()),
        };
        channel.close()?;
        channel.wait_close()?;
        self.channel = None;
        Ok(())
    }

    /// Dispatches the input data over connected SSH stream.
    /// Returns the String containing whole reponse received from server up to & excluding the NETCONF message separator.
    pub fn dispatch_netconf_request(&mut self, request: &impl NetconfRequest) -> Result<String> {
        if self.channel.is_some() {
            let cmd = request.to_raw_xml()? + MESSAGE_SEPARATOR;
            self.write_all(cmd.as_bytes())?;
            let res = self.get_reply()?;
            Ok(res)
        } else {
            bail!("request: Channel not connected!");
        }
    }

    /// Blocking receive of the stream data. Buffers all the incoming data up until NETCONF message terminator.
    fn get_reply(&mut self) -> Result<String> {
        if self.channel.is_none() {
            bail!("response: Channel not connected!");
        }

        let mut result = String::new();
        loop {
            let mut buffer = [1u8; 1024];
            let bytes_read = self.read(&mut buffer)?;
            let s = String::from_utf8_lossy(&buffer[..bytes_read]);
            result.push_str(&s);

            if result.ends_with(MESSAGE_SEPARATOR) {
                break;
            }
            if result.ends_with("##") {
                break;
            }

            if bytes_read == 0 || self.channel.as_ref().unwrap().eof() {
                bail!("Buffer is empty, SSH channel read terminated");
            }
        }
        if let Some(stripped) = result.strip_suffix(MESSAGE_SEPARATOR) {
            result = stripped.to_string();
        }
        Ok(result)
    }
}

impl Drop for SshClient {
    fn drop(&mut self) {
        if self.channel.is_some() {
            if let Result::Err(err) = self.disconnect() {
                println!("SSH disconnect error: {}", err);
            }
        }
    }
}

impl Write for SshClient {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.channel.as_mut().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.channel.as_mut().unwrap().flush()
    }
}

impl Read for SshClient {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.channel.as_mut().unwrap().read(buf)
    }
}
