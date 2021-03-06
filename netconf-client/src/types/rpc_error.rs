// TODO - validate error payload - https://datatracker.ietf.org/doc/html/rfc6241#appendix-A

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(from = "RpcErrorRpc")]
pub struct RpcError {
    pub error_type: ErrorType,
    pub error_tag: String,
    pub error_severity: ErrorSeverity,
    pub error_app_tag: Option<String>,
    pub error_path: Option<String>, // TODO - XPath dedicated type?
    pub error_message: Option<String>,
    pub error_info: Option<String>,
}

impl From<RpcErrorRpc> for RpcError {
    fn from(rpc: RpcErrorRpc) -> Self {
        RpcError {
            error_type: rpc.error_type.item,
            error_tag: rpc.error_tag.clone(),
            error_severity: rpc.error_severity.item,
            error_app_tag: rpc.error_app_tag.clone(),
            error_path: rpc.error_path.clone(),
            error_message: rpc.error_message.clone(),
            error_info: rpc.error_info,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct RpcErrorRpc {
    error_type: ErrorTypeRpc,
    error_tag: String,
    error_severity: ErrorSeverityRpc,
    // #[serde(skip_serializing_if = "Option::is_none")]
    error_app_tag: Option<String>,
    error_path: Option<String>, // TODO - XPath dedicated type?
    error_message: Option<String>,
    // TODO - implement  custom nested tags causing de/se failure
    #[serde(skip)]
    error_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ErrorTypeRpc {
    #[serde(rename = "$value")]
    item: ErrorType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ErrorType {
    Transport,
    Rpc,
    Protocol,
    Application,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ErrorSeverity {
    Error,
    Warning,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ErrorSeverityRpc {
    #[serde(rename = "$value")]
    item: ErrorSeverity,
}
