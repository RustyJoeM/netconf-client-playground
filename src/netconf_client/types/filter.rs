use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(into = "FilterRpc")]
pub struct Filter {
    pub filter_type: FilterType,
    pub data: String,
}

impl From<Filter> for FilterRpc {
    fn from(request: Filter) -> Self {
        FilterRpc {
            filter: request.filter_type.to_string(),
            data: request.data,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "filter")]
pub(in crate::netconf_client) struct FilterRpc {
    #[serde(rename = "type")]
    filter: String,
    #[serde(rename = "$value")]
    data: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FilterType {
    Subtree,
    /// Usable only when client supports the [`Capability::XPath`] capability. (see [RFC 6241](https://datatracker.ietf.org/doc/html/rfc6241#section-8.9), section 8.9)
    XPath,
}

// Needed because of current quick-xml inability to serialize our structure properly for enum (must be String).
impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FilterType::Subtree => "subtree",
            FilterType::XPath => "xpath",
        };
        write!(f, "{}", s)
    }
}
