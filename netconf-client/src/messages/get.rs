use std::fmt::Debug;

use anyhow::{bail, Result};
use quick_xml::{
    de::from_str,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
use serde::Deserialize;

use crate::{
    common::{get_tag_slice, xml_events_to_string, RpcWrapMode, XMLNS},
    types::{Filter, RpcErrorRpc, RpcReply},
};

#[derive(Debug, Clone)]
pub struct GetRequest {
    pub message_id: String,
    pub xmlns: String,
    pub filter: Option<Filter>,
}

impl GetRequest {
    pub fn new(message_id: String, filter: Option<Filter>) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            filter,
        }
    }
}

impl super::NetconfRequest for GetRequest {
    fn to_netconf_rpc(&self) -> Result<String> {
        // TODO - might move also root tag into `xml_events_to_string` if no usage has attributes?
        let mut events = vec![Event::Start(BytesStart::borrowed(b"get", b"get".len()))];

        if let Some(filter) = &self.filter {
            let filter_str = filter.to_netconf_rpc()?;
            events.push(Event::Text(BytesText::from_escaped_str(filter_str)));
        }
        events.push(Event::End(BytesEnd::borrowed(b"get")));

        xml_events_to_string(&events, RpcWrapMode::Wrapped(&self.message_id, &self.xmlns))
    }
}

#[derive(Debug)]
pub struct GetResponse {
    full_dump: String,
    pub message_id: String,
    pub xmlns: String,
    pub reply: RpcReply,
}

#[derive(Debug, Deserialize, Clone)]
struct GetResponseRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "rpc-error")]
    rpc_error: Option<RpcErrorRpc>,
}

impl GetResponse {
    pub fn data(&self) -> Result<&str> {
        match self.reply {
            RpcReply::Ok => get_tag_slice(&self.full_dump, "data"),
            RpcReply::Error(_) => bail!("No data in error reply"),
        }
    }
}

impl super::NetconfResponse for GetResponse {
    fn from_netconf_rpc(s: String) -> Result<Self>
    where
        Self: Sized,
    {
        let rpc: GetResponseRpc = from_str(&s)?;
        let message_id = rpc.message_id;
        let xmlns = rpc.xmlns;
        let reply = match rpc.rpc_error {
            None => RpcReply::Ok,
            Some(err) => RpcReply::Error(err.into()),
        };
        Ok(Self {
            full_dump: s,
            message_id,
            xmlns,
            reply,
        })
    }
}
