use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RpcError {
    error_type: ErrorType,
    error_tag: String,
    error_severity: ErrorSeverity,
    error_app_tag: Option<String>,
    error_path: Option<String>, // TODO - XPath dedicated type?
    error_message: Option<String>,
    error_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ErrorType {
    Transport,
    Rpc,
    Protocol,
    Application,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ErrorSeverity {
    Error,
    Warning,
}

// TODO - validate error payload - https://datatracker.ietf.org/doc/html/rfc6241#appendix-A
