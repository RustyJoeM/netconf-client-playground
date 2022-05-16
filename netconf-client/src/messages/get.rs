use std::fmt::Debug;

use anyhow::{bail, Result};
use quick_xml::{
    de::from_str,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
use serde::Deserialize;

use crate::{
    common::{get_tag_slice, xml_events_to_string, RpcWrapMode, XMLNS},
    types::{FilterPayload, RpcErrorRpc, RpcReply},
};

use super::{FullResponse, NetconfRequest, NetconfResponse, ToPrettyXml, ToRawXml};

#[derive(Debug, Clone)]
pub struct GetRequest {
    pub message_id: String,
    pub xmlns: String,
    pub filter: Option<FilterPayload>,
}

impl GetRequest {
    pub fn new(message_id: String, filter: Option<FilterPayload>) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            filter,
        }
    }
}

impl ToRawXml for GetRequest {
    fn to_raw_xml(&self) -> Result<String> {
        // TODO - might move also root tag into `xml_events_to_string` if no usage has attributes?
        let mut events = vec![Event::Start(BytesStart::borrowed(b"get", b"get".len()))];

        if let Some(filter) = &self.filter {
            let filter_str = filter.to_raw_xml()?;
            events.push(Event::Text(BytesText::from_escaped_str(filter_str)));
        }
        events.push(Event::End(BytesEnd::borrowed(b"get")));

        xml_events_to_string(&events, RpcWrapMode::Wrapped(&self.message_id, &self.xmlns))
    }
}

impl ToPrettyXml for GetRequest {}

impl NetconfRequest for GetRequest {
    type Response = GetResponse;
}

#[derive(Debug)]
pub struct GetResponse {
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

impl FullResponse<GetResponse> {
    pub fn data(&self) -> Result<&str> {
        match self.typed.reply {
            RpcReply::Ok => get_tag_slice(&self.dump, "data"),
            RpcReply::Error(_) => bail!("No data in error reply"),
        }
    }
}

impl NetconfResponse for GetResponse {
    fn from_netconf_rpc(s: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let rpc: GetResponseRpc = from_str(s)?;
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

    fn succeeded(&self) -> bool {
        self.reply == RpcReply::Ok
    }
}
