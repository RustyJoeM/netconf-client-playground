use std::fmt::Debug;

use anyhow::{bail, Result};
use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};

use crate::netconf::{
    common::{get_tag_slice, XMLNS},
    types::{tag_wrapper::TagWrapper, Datastore, Filter, RpcErrorRpc, RpcReply},
};

// TODO - url parameter - https://datatracker.ietf.org/doc/html/rfc6241#section-8.8

/// TODO - operation for \<edit-config\> elements, but not used due to nested generic XML here...
pub enum Operation {
    Merge,
    Replace,
    Create,
    Delete,
    Remove,
}

#[derive(Debug, Serialize, Clone)]
pub enum DefaultOperation {
    Merge,
    Replace,
    None,
}

#[derive(Debug, Serialize, Clone)]
pub enum TestOption {
    TestThenSet,
    Set,
    TestOnly,
}

#[derive(Debug, Serialize, Clone)]
pub enum ErrorOption {
    StopOnError,
    ContinueOnError,
    RollbackOnError,
}

#[derive(Debug, Serialize, Clone)]
#[serde(into = "EditConfigRequestRpc")]
pub struct EditConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub params: EditConfigParams,
}

#[derive(Debug, Clone)]
pub struct EditConfigParams {
    pub target: Datastore,
    pub default_operation: Option<DefaultOperation>,
    pub test_option: Option<TestOption>,
    pub error_option: Option<ErrorOption>,
    pub config: String, // TODO - url instead of config (see TODO rfc ref above)
}

impl From<EditConfigRequest> for EditConfigRequestRpc {
    fn from(request: EditConfigRequest) -> Self {
        EditConfigRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            edit_config: request.params.into(),
        }
    }
}

impl From<EditConfigParams> for EditConfigRpc {
    fn from(params: EditConfigParams) -> Self {
        Self {
            target: TagWrapper::new(params.target),
            default_operation: params.default_operation.map(TagWrapper::new),
            test_option: params.test_option.map(TagWrapper::new),
            error_option: params.error_option.map(TagWrapper::new),
            config: TagWrapper::new(params.config),
        }
    }
}

impl EditConfigRequest {
    fn new(message_id: String, params: EditConfigParams) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            params,
        }
    }

    pub fn new_request_str(message_id: String, params: EditConfigParams) -> Result<String> {
        const TOKEN: &str = "MAGIC_TOKEN";
        let mut params = params;

        // reset <config> contents for automatic serialization to a TOKEN to be replaced later
        let config_str = params.config;
        params.config = TOKEN.to_string();

        // serialize RPC without <config> data
        let instance = Self::new(message_id, params);
        let mut instance_str = to_string(&instance)?;
        // replace back the original filter data (auto would have escaped tags to html &lt; / &gt;)
        instance_str = instance_str.replace(TOKEN, &config_str);
        Ok(instance_str)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct EditConfigRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "edit-config")]
    edit_config: EditConfigRpc,
}

#[derive(Debug, Serialize)]
#[serde(rename = "edit-config")]
struct EditConfigRpc {
    target: TagWrapper<Datastore>,
    #[serde(rename = "default-operation")]
    default_operation: Option<TagWrapper<DefaultOperation>>,
    #[serde(rename = "test-option")]
    test_option: Option<TagWrapper<TestOption>>,
    #[serde(rename = "error-option")]
    error_option: Option<TagWrapper<ErrorOption>>,
    config: TagWrapper<String>, // TODO - url instead of config (see TODO rfc ref above)
}

#[derive(Debug, Serialize)]
struct ConfigRpc {
    #[serde(rename = "$value")]
    item: String,
}

#[derive(Debug, Serialize)]
struct TargetRpc {
    #[serde(rename = "$value")]
    item: Datastore,
}

// #[derive(Debug)]
// pub struct GetConfigResponse {
//     full_dump: String,
//     pub message_id: String,
//     pub xmlns: String,
//     pub reply: RpcReply,
// }

// #[derive(Debug, Deserialize, Clone)]
// struct GetConfigResponseRpc {
//     #[serde(rename = "message-id")]
//     message_id: String,
//     xmlns: String,
//     #[serde(rename = "rpc-error")]
//     rpc_error: Option<RpcErrorRpc>,
// }

// impl GetConfigResponse {
//     pub fn from_str(s: String) -> Result<Self> {
//         let rpc: GetConfigResponseRpc = from_str(&s)?;
//         let message_id = rpc.message_id;
//         let xmlns = rpc.xmlns;
//         let reply = match rpc.rpc_error {
//             None => RpcReply::Ok,
//             Some(err) => RpcReply::Error(err.into()),
//         };
//         Ok(Self {
//             full_dump: s,
//             message_id,
//             xmlns,
//             reply,
//         })
//     }

//     pub fn data(&self) -> Result<&str> {
//         match self.reply {
//             RpcReply::Ok => get_tag_slice(&self.full_dump, "data"),
//             RpcReply::Error(_) => bail!("No data in error reply"),
//         }
//     }
// }
