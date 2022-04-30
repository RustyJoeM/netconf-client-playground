pub mod cancel_commit;
pub mod close_session;
pub mod commit;
pub mod copy_config;
pub mod delete_config;
pub mod discard_changes;
pub mod edit_config;
pub mod get;
pub mod get_config;
pub mod hello;
pub mod kill_session;
pub mod lock;
pub mod unlock;
pub mod validate;

use anyhow::Result;
use std::fmt::Debug;

pub trait NetconfRequest {
    type Response: NetconfResponse + Debug;

    fn to_netconf_rpc(&self) -> Result<String>;

    fn prettified_rpc(&self) -> Result<String> {
        let raw_rpc = self.to_netconf_rpc()?;
        prettified_rpc(&raw_rpc)
    }
}

#[derive(Debug)]
pub struct FullResponse<R: NetconfResponse + Debug> {
    pub typed: R,
    pub dump: String,
}

impl<R: NetconfResponse + Debug> FullResponse<R> {
    pub fn prettified_rpc(&self) -> Result<String> {
        let pretty = prettified_rpc(&self.dump)?;
        Ok(pretty)
    }
}

pub trait NetconfResponse {
    fn from_netconf_rpc(s: &str) -> Result<Self>
    where
        Self: Sized;

    fn succeeded(&self) -> bool {
        true
    }
}

fn prettified_rpc(rpc_str: &str) -> Result<String> {
    let mut reader = quick_xml::Reader::from_str(rpc_str);
    reader.trim_text(true);

    let mut writer = quick_xml::Writer::new_with_indent(Vec::new(), b' ', 2);
    let mut read_buf = Vec::new();
    loop {
        match reader.read_event(&mut read_buf) {
            Ok(quick_xml::events::Event::Eof) => {
                break;
            }
            Ok(event) => {
                writer.write_event(event)?;
            }
            Err(e) => {
                anyhow::bail!("Error at position {}: {:?}", reader.buffer_position(), e);
            }
        };

        read_buf.clear();
    }

    let result = String::from_utf8_lossy(&*writer.into_inner()).to_string();
    Ok(result)
}
