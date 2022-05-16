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

use crate::types::Capability;

pub trait ToRawXml {
    fn to_raw_xml(&self) -> Result<String>;
}

impl<T> ToRawXml for T
where
    T: serde::Serialize,
{
    fn to_raw_xml(&self) -> Result<String> {
        let raw_xml = quick_xml::se::to_string(&self)?;
        Ok(raw_xml)
    }
}

pub trait ToPrettyXml: ToRawXml {
    fn to_pretty_xml(&self) -> Result<String> {
        let raw_rpc = self.to_raw_xml()?;
        raw_to_pretty_xml(&raw_rpc)
    }
}

impl<T> ToPrettyXml for T where T: serde::Serialize {}

pub trait NetconfRequest: ToPrettyXml + Debug {
    type Response: NetconfResponse;

    /// Perform a check of request payload against server capabilities.
    ///
    /// Returns `Ok(())` on success, or `Err(_)` with problem description otherwise.
    fn validate_request(&self, _server_capabilities: &[Capability]) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FullResponse<R: NetconfResponse> {
    pub typed: R,
    pub dump: String,
}

impl<R: NetconfResponse> ToRawXml for FullResponse<R> {
    // TODO - Cow?
    fn to_raw_xml(&self) -> Result<String> {
        Ok(self.dump.clone())
    }
}

impl<R: NetconfResponse> ToPrettyXml for FullResponse<R> {}

pub trait NetconfResponse: Debug {
    fn from_netconf_rpc(s: &str) -> Result<Self>
    where
        Self: Sized;

    fn succeeded(&self) -> bool {
        true
    }
}

pub fn raw_to_pretty_xml(rpc_str: &str) -> Result<String> {
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
