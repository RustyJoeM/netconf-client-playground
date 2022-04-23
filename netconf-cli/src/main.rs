// #![deny(missing_docs)]
// #![warn(missing_docs)]
// #![deny(missing_doc_code_examples)]

use anyhow::Result;

use netconf_client::{types::Capability, NetconfSession, SshAuthentication};

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

    // use netconf_client::types::{Datastore, Filter, FilterType};
    // let filter = Filter {
    //     // value: FilterType::Subtree(
    //     //     "<dhcp xmlns=\"http://tail-f.com/ns/example/dhcpd\"/>".to_string(),
    //     // ),
    //     value: FilterType::Xpath("/dhcp/something[a = 123]".to_string()),
    //     namespaces: vec![
    //         // ("xmlns:first".to_string(), "todo".to_string()),
    //         ("xmlns".to_string(), "bbbbb".to_string()),
    //     ],
    // };
    // // dbg!(&filter.to_netconf_rpc()?);
    // dbg!(session.request_get_config(Datastore::Running, Some(filter))?);

    // use netconf_client::types::Datastore;
    // let res = session.request_get_config(Datastore::Running, Some(filter))?;
    // dbg!(res.data()?);

    // dbg!(session.request_commit()?);

    // use netconf_client::messages::commit;
    // let params = commit::ConfirmedCommitParams {
    //     confirm_timeout: Some(30),
    //     persist: Some("abra".to_string()),
    //     persist_id: Some("cadabra".to_string()),
    // };
    // dbg!(session.request_confirmed_commit(params)?);

    // use netconf_client::messages::edit_config;
    // let params = edit_config::EditConfigParams {
    //     target: Datastore::Running,
    //     default_operation: None,
    //     test_option: None,
    //     error_option: None,
    //     // config: edit_config::EditConfigContent::Config("<right/>".to_string()),
    //     config: edit_config::EditConfigContent::Url("https://example.com/test".to_string()),
    // };
    // dbg!(session.request_edit_config(params)?);

    // use netconf_client::messages::validate;
    // let source = validate::ValidateSource::Datastore(Datastore::Running);
    // dbg!(session.request_validate(source)?);

    // dbg!(session.request_discard_changes()?);
    // dbg!(session.request_kill_session(11)?);
    dbg!(session.request_close_session()?);

    // let urn = "urn:ietf:params:netconf:capability:url:1.0?scheme=http,ftp,file".to_string();
    // let cap = Capability::from_urn(&urn)?;
    // dbg!(&cap);

    Ok(())
}
