use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, SimpleResponse},
};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "KillSessionRequestRpc")]
pub struct KillSessionRequest {
    pub message_id: String,
    pub xmlns: String,
    pub session_id: u32,
}

impl super::NetconfRequest for KillSessionRequest {
    fn to_netconf_rpc(&self) -> anyhow::Result<String> {
        let res = quick_xml::se::to_string(&self)?;
        Ok(res)
    }
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

pub type KillSessionResponse = SimpleResponse;
