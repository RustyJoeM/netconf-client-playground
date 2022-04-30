use anyhow::{bail, Result};
use serde::Deserialize;

use crate::{
    messages::NetconfResponse,
    types::{RpcError, RpcReply},
};

/// Simple response type used by several NETCONF operations,
/// when NETCONF server returns either:
///  - \<ok/\> tag on success
///  - \<rpc-error\> on failure
#[derive(Debug, Deserialize)]
#[serde(try_from = "SimpleResponseRpc")]
pub struct SimpleResponse {
    pub message_id: String,
    pub xmlns: String,
    pub reply: RpcReply,
}

/// Private RPC representation of the [SimpleResponse].
#[derive(Debug, Deserialize)]
#[serde(rename = "rpc")]
struct SimpleResponseRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    ok: Option<()>,
    #[serde(rename = "rpc-error")]
    rpc_error: Option<RpcError>,
}

impl TryFrom<SimpleResponseRpc> for SimpleResponse {
    type Error = anyhow::Error;

    fn try_from(value: SimpleResponseRpc) -> Result<Self> {
        let message_id = value.message_id;
        let xmlns = value.xmlns;
        let reply = match value.ok.is_some() {
            true => RpcReply::Ok,
            false => match value.rpc_error {
                Some(err) => RpcReply::Error(err),
                None => bail!("Missing both <ok/> and <rpc-error> from response"),
            },
        };
        Ok(SimpleResponse {
            message_id,
            xmlns,
            reply,
        })
    }
}

impl NetconfResponse for SimpleResponse {
    fn from_netconf_rpc(s: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let res: Self = quick_xml::de::from_str(s)?;
        Ok(res)
    }

    fn succeeded(&self) -> bool {
        self.reply == RpcReply::Ok
    }
}
