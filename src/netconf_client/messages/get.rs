use std::fmt::Debug;

use anyhow::{bail, Result};
use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};

use crate::netconf_client::{
    common::{get_tag_slice, XMLNS},
    types::{Filter, FilterRpc, RpcErrorRpc, RpcReply},
};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "GetRequestRpc")]
pub struct GetRequest {
    pub message_id: String,
    pub xmlns: String,
    pub filter: Option<Filter>,
}

impl From<GetRequest> for GetRequestRpc {
    fn from(request: GetRequest) -> Self {
        GetRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            get: GetRpc {
                filter: request.filter.map(|f| f.into()),
            },
        }
    }
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
        const TOKEN: &str = "MAGIC_TOKEN";
        let mut filter = self.filter.clone();

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
        let instance = Self::new(self.message_id.clone(), filter);
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
struct GetRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    get: GetRpc,
}

#[derive(Debug, Serialize)]
#[serde(rename = "get")]
struct GetRpc {
    filter: Option<FilterRpc>,
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
    pub fn from_str(s: String) -> Result<Self> {
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

    pub fn data(&self) -> Result<&str> {
        match self.reply {
            RpcReply::Ok => get_tag_slice(&self.full_dump, "data"),
            RpcReply::Error(_) => bail!("No data in error reply"),
        }
    }
}
