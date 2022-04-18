// #![deny(missing_docs)]
// #![warn(missing_docs)]
// #![deny(missing_doc_code_examples)]

use anyhow::Result;

mod netconf_client;
use crate::netconf_client::{types::Capability, NetconfSession, SshAuthentication};

fn main() -> Result<()> {
    let mut session = NetconfSession::initialize(
        "127.0.0.1".parse()?,
        2022,
        SshAuthentication::UserPassword("admin".to_string(), "admin".to_string()),
        vec![Capability::Base],
    )?;
    session.set_validate_capabilities(false);

    // dbg!(session.request_lock(Datastore::Running)?);
    // dbg!(session.request_unlock(Datastore::Running)?);
    // dbg!(session.request_get(None)?);

    // use crate::netconf::types::{Filter, FilterType};
    // let filter = Filter {
    //     filter_type: FilterType::Subtree,
    //     data: "<dhcp xmlns=\"http://tail-f.com/ns/example/dhcpd\"/>".to_string(),
    // };
    // // let res = session.request_get(Some(filter))?;
    // use crate::netconf::types::Datastore;
    // let res = session.request_get_config(Datastore::Running, Some(filter))?;
    // dbg!(res.data()?);

    // dbg!(session.request_commit()?);

    // use crate::netconf_client::ConfirmedCommitParams;
    // let params = ConfirmedCommitParams {
    //     confirm_timeout: Some(30),
    //     persist: Some("abra".to_string()),
    //     persist_id: Some("cadabra".to_string()),
    // };
    // dbg!(session.request_confirmed_commit(params)?);

    // dbg!(session.request_discard_changes()?);
    // dbg!(session.kill_session(11)?);
    dbg!(session.request_close_session()?);

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
