// #![deny(missing_docs)]
// #![warn(missing_docs)]
// #![deny(missing_doc_code_examples)]

use anyhow::Result;

mod netconf_client;
use crate::netconf_client::{
    types::{Capability, Datastore},
    NetconfSession, SshAuthentication,
};

fn main() -> Result<()> {
    let mut client = NetconfSession::new(
        "127.0.0.1".parse()?,
        2022,
        SshAuthentication::UserPassword("admin".to_string(), "admin".to_string()),
        vec![Capability::Base],
    );
    client.set_validate_capabilities(false);

    dbg!(client.connect()?);
    dbg!(client.request_hello()?);
    // dbg!(client.request_lock(Datastore::Running)?);
    // dbg!(client.request_unlock(Datastore::Running)?);
    // dbg!(client.get(None)?);

    // use crate::netconf::types::{Filter, FilterType};
    // let filter = Filter {
    //     filter_type: FilterType::Subtree,
    //     data: "<dhcp xmlns=\"http://tail-f.com/ns/example/dhcpd\"/>".to_string(),
    // };
    // // let res = client.get(Some(filter))?;
    // use crate::netconf::types::Datastore;
    // let res = client.get_config(Datastore::Running, Some(filter))?;
    // dbg!(res.data()?);

    // dbg!(client.request_commit()?);
    // dbg!(client.request_discard_changes()?);
    // dbg!(client.kill_session(11)?);
    dbg!(client.request_close_session()?);

    // use netconf::{
    //     messages::edit_config::{EditConfigParams, EditConfigRequest},
    //     types::Datastore,
    // };
    // let params = EditConfigParams {
    //     target: Datastore::Running,
    //     default_operation: None,
    //     test_option: None,
    //     error_option: None,
    //     config: "<right/>".to_string(),
    // };
    // let req = EditConfigRequest::new_request_str("123".to_string(), params)?;
    // dbg!(req);

    // let req = GetRequest::new("123".to_string(), Some(filter));
    // dbg!(quick_xml::se::to_string(&req));

    Ok(())
}
