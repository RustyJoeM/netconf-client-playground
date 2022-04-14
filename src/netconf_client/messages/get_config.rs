use std::fmt::Debug;

use anyhow::{bail, Result};
use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};

use crate::netconf_client::{
    common::{get_tag_slice, XMLNS},
    types::{Datastore, Filter, FilterRpc, RpcErrorRpc, RpcReply},
};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "GetConfigRequestRpc")]
pub struct GetConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub source: Datastore,
    pub filter: Option<Filter>,
}

impl From<GetConfigRequest> for GetConfigRequestRpc {
    fn from(request: GetConfigRequest) -> Self {
        GetConfigRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            get_config: GetConfigRpc {
                source: SourceRpc {
                    item: request.source,
                },
                filter: request.filter.map(|f| f.into()),
            },
        }
    }
}

impl GetConfigRequest {
    fn new(message_id: String, source: Datastore, filter: Option<Filter>) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            source,
            filter,
        }
    }

    pub fn new_request_str(
        message_id: String,
        source: Datastore,
        filter: Option<Filter>,
    ) -> Result<String> {
        const TOKEN: &str = "MAGIC_TOKEN";
        let mut filter = filter;

        // extract user-defined filter data
        let filter_str: Option<String> = match &mut filter {
            Some(f) => {
                let res = Some(f.data.clone());
                // reset it for automatic serialization to a TOKEN to be replaced later
                f.data = TOKEN.to_string();
                res
            }
            None => None,
        };

        // serialize RPC without filter data (if some)
        let instance = Self::new(message_id, source, filter);
        let mut instance_str = to_string(&instance)?;

        // replace back the original filter data (auto would have escaped tags to html &lt; / &gt;)
        if let Some(f) = filter_str {
            instance_str = instance_str.replace(TOKEN, &f);
        }
        Ok(instance_str)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct GetConfigRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "get-config")]
    get_config: GetConfigRpc,
}

#[derive(Debug, Serialize)]
#[serde(rename = "get-config")]
struct GetConfigRpc {
    source: SourceRpc,
    filter: Option<FilterRpc>,
}

#[derive(Debug, Serialize)]
struct SourceRpc {
    #[serde(rename = "$value")]
    item: Datastore,
}

#[derive(Debug)]
pub struct GetConfigResponse {
    full_dump: String,
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

impl GetConfigResponse {
    pub fn from_str(s: String) -> Result<Self> {
        let rpc: GetConfigResponseRpc = from_str(&s)?;
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

    pub fn data(&self) -> Result<&str> {
        match self.reply {
            RpcReply::Ok => get_tag_slice(&self.full_dump, "data"),
            RpcReply::Error(_) => bail!("No data in error reply"),
        }
    }
}
