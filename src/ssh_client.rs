use anyhow::{bail, Result};
use ssh2::{Channel, Session};
use std::{
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpStream},
    str::FromStr,
};

use crate::netconf::common::MESSAGE_SEPARATOR;

pub enum SshAuthentication {
    UserPassword(String, String),
}

pub struct SshClient {
    address: String,
    port: u16,
    auth: SshAuthentication,
    channel: Option<Channel>,
}

impl SshClient {
    pub fn new(address: &str, port: u16, auth: SshAuthentication) -> Self {
        Self {
            address: address.to_string(),
            port,
            auth,
            channel: None,
        }
    }

    pub fn dispatch_xml_request(&mut self, data: &str) -> Result<String> {
        if self.channel.is_some() {
            self.send_str(data)?;
            let res = self.get_reply()?;
            Ok(res)
        } else {
            bail!("request: Channel not connected!");
        }
    }

    fn send_str(&mut self, data: &str) -> Result<()> {
        let cmd = data.to_string() + MESSAGE_SEPARATOR;
        self.write_all(cmd.as_bytes())?;
        Ok(())
    }

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
                // if self.channel.as_ref().unwrap().eof() {
                // if bytes_read == 0 {
                bail!("Buffer is empty, SSH channel read terminated");
                // } else {
                //     break; // read some response and closed socket
                // }
            }
            // if bytes_read == 0 {
            //     return Err(std::io::Error::new(
            //         io::ErrorKind::UnexpectedEof,
            //         "Buffer is empty, SSH channel read terminated".to_string(),
            //     )
            //     .into());
            // }
        }
        Ok(result)
    }

    pub fn connect(&mut self) -> Result<()> {
        let ip_addr = IpAddr::from_str(&self.address)?;
        let socket_address = SocketAddr::from((ip_addr, self.port));
        let tcp = TcpStream::connect(socket_address)?;

        let mut session = Session::new()?;

        session.set_blocking(true);
        session.set_timeout(5000); // TODO magic constant
        session.set_tcp_stream(tcp);
        session.handshake()?;

        match &self.auth {
            SshAuthentication::UserPassword(username, password) => {
                session.userauth_password(username, password)?;
            }
        }

        // let mut channel = session.channel_session()?;
        // let mut channel = session.channel_direct_tcpip(&self.address, self.port, None)?;
        let mut channel = session.channel_session()?;
        channel.subsystem("netconf")?;
        // channel.shell()?;
        self.channel = Some(channel);

        Ok(())
    }

    pub fn drop_channel(&mut self) {
        self.channel = None;
    }

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
        self.drop_channel();
        Ok(())
    }
}

impl Drop for SshClient {
    fn drop(&mut self) {
        dbg!("dropping SSH");
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
