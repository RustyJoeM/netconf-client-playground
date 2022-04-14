use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::netconf::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, Datastore, RpcErrorRpc, RpcReply},
};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "CopyConfigRequestRpc")]
pub struct CopyConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub target: Datastore,
    pub source: Datastore,
}

impl From<CopyConfigRequest> for CopyConfigRequestRpc {
    fn from(request: CopyConfigRequest) -> Self {
        CopyConfigRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            copy_config: CopyConfigRpc {
                target: request.target.into(),
                source: request.source.into(),
            },
        }
    }
}

impl CopyConfigRequest {
    pub fn new(message_id: String, target: Datastore, source: Datastore) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            target,
            source,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct CopyConfigRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "copy-config")]
    copy_config: CopyConfigRpc,
}

#[derive(Debug, Serialize)]
#[serde(rename = "copy-config")]
struct CopyConfigRpc {
    target: TagWrapper<Datastore>,
    source: TagWrapper<Datastore>,
}

#[derive(Debug, Deserialize)]
#[serde(from = "CopyConfigResponseRpc")]
pub struct CopyConfigResponse {
    pub message_id: String,
    pub xmlns: String,
    pub reply: RpcReply,
}

#[derive(Debug, Deserialize, Clone)]
struct CopyConfigResponseRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    ok: Option<()>,
    #[serde(rename = "rpc-error")]
    rpc_error: Option<RpcErrorRpc>,
}

impl From<CopyConfigResponseRpc> for CopyConfigResponse {
    fn from(rpc: CopyConfigResponseRpc) -> Self {
        CopyConfigResponse {
            message_id: rpc.message_id,
            xmlns: rpc.xmlns,
            reply: match rpc.rpc_error {
                None => RpcReply::Ok,
                Some(err) => RpcReply::Error(err.into()),
            },
        }
    }
}
