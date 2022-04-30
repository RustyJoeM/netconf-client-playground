use std::fmt::Debug;

use anyhow::{bail, Result};
use quick_xml::{
    de::from_str,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
use serde::Deserialize;

use crate::{
    common::{get_tag_slice, xml_events_to_string, RpcWrapMode, XMLNS},
    types::{Datastore, Filter, RpcErrorRpc, RpcReply},
};

use super::{FullResponse, NetconfResponse};

#[derive(Debug, Clone)]
pub struct GetConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub source: Datastore,
    pub filter: Option<Filter>,
}

impl GetConfigRequest {
    pub fn new(message_id: String, source: Datastore, filter: Option<Filter>) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            source,
            filter,
        }
    }
}

impl super::NetconfRequest for GetConfigRequest {
    type Response = GetConfigResponse;

    fn to_netconf_rpc(&self) -> Result<String> {
        let source_str = self.source.to_string();

        let mut events = vec![
            Event::Start(BytesStart::borrowed(b"get-config", b"get-config".len())),
            Event::Start(BytesStart::borrowed(b"source", b"source".len())),
            Event::Empty(BytesStart::borrowed(
                source_str.as_bytes(),
                source_str.len(),
            )),
            Event::End(BytesEnd::borrowed(b"source")),
        ];

        if let Some(filter) = &self.filter {
            let filter_str = filter.to_netconf_rpc()?;
            events.push(Event::Text(BytesText::from_escaped_str(filter_str)));
        }
        events.push(Event::End(BytesEnd::borrowed(b"get-config")));

        xml_events_to_string(&events, RpcWrapMode::Wrapped(&self.message_id, &self.xmlns))
    }
}

#[derive(Debug)]
pub struct GetConfigResponse {
    pub message_id: String,
    pub xmlns: String,
    pub reply: RpcReply,
}

#[derive(Debug, Deserialize, Clone)]
struct GetConfigResponseRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "rpc-error")]
    rpc_error: Option<RpcErrorRpc>,
}

impl NetconfResponse for GetConfigResponse {
    fn from_netconf_rpc(s: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let rpc: GetConfigResponseRpc = from_str(s)?;
        let message_id = rpc.message_id;
        let xmlns = rpc.xmlns;
        let reply = match rpc.rpc_error {
            None => RpcReply::Ok,
            Some(err) => RpcReply::Error(err.into()),
        };
        Ok(Self {
            message_id,
            xmlns,
            reply,
        })
    }
}

impl FullResponse<GetConfigResponse> {
    pub fn data(&self) -> Result<&str> {
        match self.typed.reply {
            RpcReply::Ok => get_tag_slice(&self.dump, "data"),
            RpcReply::Error(_) => bail!("No data in error reply"),
        }
    }
}
