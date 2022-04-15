use std::fmt::Debug;

use serde::Serialize;

use crate::netconf_client::{common::XMLNS, types::SimpleResponse};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "CommitRequestRpc")]
pub struct CommitRequest {
    pub message_id: String,
    pub xmlns: String,
}

impl From<CommitRequest> for CommitRequestRpc {
    fn from(request: CommitRequest) -> Self {
        CommitRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            commit: CommitRpc {},
        }
    }
}

impl CommitRequest {
    pub fn new(message_id: String) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct CommitRpc;

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct CommitRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    commit: CommitRpc,
}

pub type CommitResponse = SimpleResponse;
