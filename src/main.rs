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

    dbg!(client.connect()?);
    dbg!(client.hello()?);
    // dbg!(client.lock(netconf::types::Datastore::Running)?);
    // dbg!(client.unlock(netconf::types::Datastore::Running)?);
    // dbg!(client.get(None)?);
    // let filter = GetFilter {
    //     filter_type: FilterType::Subtree,
    //     data: "<todo/>".to_string(),
    // };
    // dbg!(client.get(Some(filter))?);
    // dbg!(client.kill_session(11)?);
    dbg!(client.close_session()?);

    // let req = GetRequest::new("123".to_string(), Some(filter));
    // dbg!(quick_xml::se::to_string(&req));

    Ok(())
}
