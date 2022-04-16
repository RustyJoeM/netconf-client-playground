use serde::Serialize;

use crate::netconf_client::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, Datastore, SimpleResponse},
};

/// The \<lock\> request for short-lived restriction of datastore access.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "UnlockRequestRpc")]
pub struct UnlockRequest {
    #[serde(rename = "message-id")]
    message_id: String,
    target: Datastore,
}

impl UnlockRequest {
    pub fn new(message_id: String, target: Datastore) -> Self {
        Self { message_id, target }
    }
}

impl super::NetconfRequest for UnlockRequest {
    fn to_netconf_rpc(&self) -> anyhow::Result<String> {
        let res = quick_xml::se::to_string(self)?;
        Ok(res)
    }
}

/// Private representation of \<lock\> RPC used for serialization.
#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct UnlockRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    unlock: UnlockRpc,
}

#[derive(Debug, Serialize)]
struct UnlockRpc {
    target: TagWrapper<Datastore>,
}

impl From<UnlockRequest> for UnlockRequestRpc {
    fn from(request: UnlockRequest) -> Self {
        UnlockRequestRpc {
            xmlns: XMLNS.to_string(),
            message_id: request.message_id,
            unlock: UnlockRpc {
                target: request.target.into(),
            },
        }
    }
}

pub type UnlockResponse = SimpleResponse;
