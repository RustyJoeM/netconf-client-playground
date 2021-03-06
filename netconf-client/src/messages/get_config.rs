use std::fmt::Debug;

use anyhow::{bail, Result};
use quick_xml::{
    de::from_str,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
use serde::Deserialize;

use crate::{
    common::{get_tag_slice, xml_events_to_string, RpcWrapMode, XMLNS},
    message_validation::validate_datastore_capability,
    types::{Capability, Datastore, FilterPayload, RpcErrorRpc, RpcReply},
};

use super::{FullResponse, NetconfRequest, NetconfResponse, ToPrettyXml, ToRawXml};

#[derive(Debug, Clone)]
pub struct GetConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub source: Datastore,
    pub filter: Option<FilterPayload>,
}

impl GetConfigRequest {
    pub fn new(message_id: String, source: Datastore, filter: Option<FilterPayload>) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            source,
            filter,
        }
    }
}

impl ToRawXml for GetConfigRequest {
    fn to_raw_xml(&self) -> Result<String> {
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
            let filter_str = filter.to_raw_xml()?;
            events.push(Event::Text(BytesText::from_escaped_str(filter_str)));
        }
        events.push(Event::End(BytesEnd::borrowed(b"get-config")));

        xml_events_to_string(&events, RpcWrapMode::Wrapped(&self.message_id, &self.xmlns))
    }
}

impl ToPrettyXml for GetConfigRequest {}

impl NetconfRequest for GetConfigRequest {
    type Response = GetConfigResponse;

    fn validate_request(&self, server_capabilities: &[crate::types::Capability]) -> Result<()> {
        validate_datastore_capability(
            &self.source,
            &Datastore::Candidate,
            &Capability::Candidate,
            server_capabilities,
        )
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

    fn succeeded(&self) -> bool {
        self.reply == RpcReply::Ok
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
