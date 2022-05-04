use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, SimpleResponse},
};

use super::NetconfRequest;

#[derive(Debug, Serialize, Clone)]
#[serde(into = "CancelCommitRequestRpc")]
pub struct CancelCommitRequest {
    pub message_id: String,
    pub xmlns: String,
    pub persist_id: Option<u32>,
}

impl NetconfRequest for CancelCommitRequest {
    type Response = CancelCommitResponse;
}

impl From<CancelCommitRequest> for CancelCommitRequestRpc {
    fn from(request: CancelCommitRequest) -> Self {
        CancelCommitRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            cancel_commit: request.persist_id.map(|x| CancelCommitRpc {
                persist_id: TagWrapper::new(x),
            }),
        }
    }
}

impl CancelCommitRequest {
    pub fn new(message_id: String, persist_id: Option<u32>) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            persist_id,
        }
    }
}

#[derive(Debug, Serialize)]
struct CancelCommitRpc {
    persist_id: TagWrapper<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct CancelCommitRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    cancel_commit: Option<CancelCommitRpc>,
}

pub type CancelCommitResponse = SimpleResponse;
