mod netconf;

mod netconf_client;
mod ssh_client;

use anyhow::Result;

use crate::{netconf_client::NetconfClient, ssh_client::SshAuthentication};

fn main() -> Result<()> {
    let mut client = NetconfClient::new(
        "127.0.0.1",
        2022,
        SshAuthentication::UserPassword("admin".to_string(), "admin".to_string()),
    );

    let response = client.connect()?;
    dbg!(&response);

    dbg!("----1");

    let response = client.hello()?;
    dbg!(&response);

    dbg!("----2");

    let response = client.hello()?;
    dbg!(&response);

    dbg!("----3");

    // std::thread::sleep(std::time::Duration::new(5, 0));

    let response = client.close_session()?;
    dbg!(&response);

    Ok(())
}
