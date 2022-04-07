use serde::Serialize;

use crate::netconf::common::XMLNS;

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
                filter: match request.filter {
                    Some(f) => Some(f.into()),
                    None => None,
                },
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
    #[serde(serialize_with = "unescaped", rename = "$value")]
    data: String,
}

fn unescaped<S>(x: &str, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x)
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FilterType {
    Subtree,
    XPath, // TODO - only if client supports capability! (RFC section 8.9)
}
