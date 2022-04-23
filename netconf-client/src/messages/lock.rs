use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, Datastore, SimpleResponse},
};

/// The \<lock\> request for short-lived restriction of datastore access.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "LockRequestRpc")]
pub struct LockRequest {
    message_id: String,
    target: Datastore,
}

impl LockRequest {
    pub fn new(message_id: String, target: Datastore) -> Self {
        Self { message_id, target }
    }
}

impl super::NetconfRequest for LockRequest {
    fn to_netconf_rpc(&self) -> anyhow::Result<String> {
        let res = quick_xml::se::to_string(self)?;
        Ok(res)
    }
}

/// Private representation of \<lock\> RPC used for serialization.
#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct LockRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    lock: LockRpc,
}

#[derive(Debug, Serialize)]
struct LockRpc {
    target: TagWrapper<Datastore>,
}

impl From<LockRequest> for LockRequestRpc {
    fn from(request: LockRequest) -> Self {
        LockRequestRpc {
            xmlns: XMLNS.to_string(),
            message_id: request.message_id,
            lock: LockRpc {
                target: request.target.into(),
            },
        }
    }
}

pub type LockResponse = SimpleResponse;
