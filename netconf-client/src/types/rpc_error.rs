// TODO - validate error payload - https://datatracker.ietf.org/doc/html/rfc6241#appendix-A

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
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
pub struct RpcErrorRpc {
    pub error_type: ErrorTypeRpc,
    pub error_tag: String,
    pub error_severity: ErrorSeverityRpc,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub error_app_tag: Option<String>,
    pub error_path: Option<String>, // TODO - XPath dedicated type?
    pub error_message: Option<String>,
    // TODO - implement  custom nested tags causing de/se failure
    #[serde(skip)]
    pub error_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorTypeRpc {
    #[serde(rename = "$value")]
    pub item: ErrorType,
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
pub struct ErrorSeverityRpc {
    #[serde(rename = "$value")]
    pub item: ErrorSeverity,
}

// #[macro_export]
// /// Helper macro to implement transition from RPC structure into
// macro_rules! deserialize_ok_response {
//     ( $source:ident, $target:ident ) => {
//         impl TryFrom<$source> for $target {
//             type Error = anyhow::Error;

//             fn try_from(value: $source) -> Result<Self> {
//                 let message_id = value.message_id;
//                 let xmlns = value.xmlns;
//                 let reply = match value.ok.is_some() {
//                     true => RpcReply::Ok,
//                     false => match value.rpc_error {
//                         Some(err) => RpcReply::Error(err.into()),
//                         None => bail!("Missing both <ok/> and <rpc-error> from response"),
//                     },
//                 };
//                 Ok($target {
//                     message_id,
//                     xmlns,
//                     reply,
//                 })
//             }
//         }
//     };
// }

// #[macro_export]
// // Helper struct for collection of random order statements.
// // each field with specified name and type gathers a vector of parsed sub-statements.
// macro_rules! unordered_stmts_storage {
//     ( $name:ident<$lifetime:lifetime>, $( ($field:ident, $type:ty) ),+ ) => {
//         #[derive(Default)]
//         struct $name<$lifetime> {
//             $( pub $field: Vec<$type>, )+
//         }
//     };
// }

// #[macro_export]
// // Simplify the parsing/collection of set of different statements in random order.
// // This stage of processing just collects statement results into "storage" arrays,
// // and does not address the min/max allowed count of variants.
// macro_rules! parse_unordered_stmts {
//     ( $i:ident, $( ($keyword:ident, $parser:expr, $storage:expr) ),+ ) => {
//         // {
//             let (_, mut next_kw) = super::keywords::peek_keyword($i)?;
//             while next_kw.is_some() {
//                 $i = match next_kw {
//                 $(
//                     Some($keyword) => {
//                         nom::combinator::map($parser, |x| $storage.push(x))($i)?.0
//                     }
//                 )+
//                     _ => {
//                         let err = nom::error::Error::new($i, nom::error::ErrorKind::Permutation);
//                         return Err(nom::Err::Error(err));
//                     }
//                 };
//                 next_kw = super::keywords::peek_keyword($i)?.1;
//             }
//         // }
//     };
// }
