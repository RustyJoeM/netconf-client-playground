use serde::{Deserialize, Serialize};

use crate::netconf::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, RpcErrorRpc, RpcReply},
};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "KillSessionRequestRpc")]
pub struct KillSessionRequest {
    pub message_id: String,
    pub xmlns: String,
    pub session_id: u32,
}

impl KillSessionRequest {
    pub fn new(message_id: String, session_id: u32) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            session_id,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
#[serde(rename_all = "kebab-case")]
struct KillSessionRequestRpc {
    message_id: String,
    xmlns: String,
    kill_session: KillSessionRpc,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct KillSessionRpc {
    session_id: TagWrapper<u32>,
}

impl From<KillSessionRequest> for KillSessionRequestRpc {
    fn from(request: KillSessionRequest) -> Self {
        KillSessionRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            kill_session: KillSessionRpc {
                session_id: request.session_id.into(),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "KillSessionResponseRpc")]
pub struct KillSessionResponse {
    pub message_id: String,
    pub xmlns: String,
    pub reply: RpcReply,
}

#[derive(Debug, Deserialize)]
struct KillSessionResponseRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    ok: Option<()>,
    #[serde(rename = "rpc-error")]
    rpc_error: Option<RpcErrorRpc>,
}

impl From<KillSessionResponseRpc> for KillSessionResponse {
    fn from(rpc: KillSessionResponseRpc) -> Self {
        KillSessionResponse {
            message_id: rpc.message_id,
            xmlns: rpc.xmlns,
            reply: match rpc.rpc_error {
                None => RpcReply::Ok,
                Some(err) => RpcReply::Error(err.into()),
            },
        }
    }
}
