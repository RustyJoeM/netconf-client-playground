use std::fmt::Debug;

use anyhow::Result;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

use crate::netconf::common::{get_tag_slice, XMLNS};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "GetRequestRpc")]
pub struct GetRequest {
    message_id: String,
    xmlns: String,
    filter: Option<GetFilter>,
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

#[derive(Debug, Serialize, Clone)]
#[serde(into = "FilterRpc")]
pub struct GetFilter {
    pub filter_type: FilterType,
    pub data: String,
}

impl From<GetFilter> for FilterRpc {
    fn from(request: GetFilter) -> Self {
        FilterRpc {
            filter: request.filter_type,
            data: request.data,
        }
    }
}

impl GetRequest {
    pub fn new(message_id: String, filter: Option<GetFilter>) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            filter,
        }
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
struct GetRpc {
    filter: Option<FilterRpc>,
}

#[derive(Debug, Serialize)]
struct FilterRpc {
    #[serde(rename = "type")]
    filter: FilterType,
    // #[serde(serialize_with = "unescaped", rename = "$value")]
    data: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FilterType {
    Subtree,
    XPath, // TODO - only if client supports capability! (RFC section 8.9)
}

#[derive(Debug)]
pub struct GetResponse {
    full_dump: String,
    pub message_id: String,
    pub xmlns: String,
}

#[derive(Debug, Deserialize, Clone)]
struct GetResponseRpc {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub xmlns: String,
}

impl GetResponse {
    pub fn from_str(s: String) -> Result<Self> {
        let rpc: GetResponseRpc = from_str(&s)?;
        let message_id = rpc.message_id;
        let xmlns = rpc.xmlns;
        Ok(Self {
            full_dump: s,
            message_id,
            xmlns,
        })
    }

    pub fn data_str(&self) -> Result<&str> {
        get_tag_slice(&self.full_dump, "data")
    }
}
