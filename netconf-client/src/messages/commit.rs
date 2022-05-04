use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, EmptyStruct, SimpleResponse},
};

use super::{NetconfRequest, ToPrettyXml, ToRawXml};

#[derive(Debug, Clone)]
pub struct CommitRequest {
    pub message_id: String,
    pub xmlns: String,
    pub params: Option<ConfirmedCommitParams>,
}

impl ToRawXml for CommitRequest {
    fn to_raw_xml(&self) -> anyhow::Result<String> {
        let res = match &self.params {
            Some(params) => quick_xml::se::to_string(&ConfirmedCommitRequestRpc {
                message_id: self.message_id.clone(),
                xmlns: self.xmlns.clone(),
                commit: ConfirmedCommitRpc {
                    confirmed: EmptyStruct {},
                    confirm_timeout: params.confirm_timeout.map(TagWrapper::new),
                    persist: params.persist.clone().map(TagWrapper::new),
                    persist_id: params.persist_id.clone().map(TagWrapper::new),
                },
            })?,
            None => quick_xml::se::to_string(&SimpleCommitRequestRpc {
                message_id: self.message_id.clone(),
                xmlns: self.xmlns.clone(),
                commit: EmptyStruct {},
            })?,
        };

        Ok(res)
    }
}

impl ToPrettyXml for CommitRequest {}

impl NetconfRequest for CommitRequest {
    type Response = CommitResponse;
}

#[derive(Debug, Clone)]
pub struct ConfirmedCommitParams {
    pub confirm_timeout: Option<u32>,
    pub persist: Option<String>,
    pub persist_id: Option<String>,
}

impl CommitRequest {
    pub fn new(message_id: String, params: Option<ConfirmedCommitParams>) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct SimpleCommitRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    commit: EmptyStruct,
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct ConfirmedCommitRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    commit: ConfirmedCommitRpc,
}

#[derive(Debug, Serialize)]
struct ConfirmedCommitRpc {
    confirmed: EmptyStruct,
    #[serde(rename = "confirm-timeout")]
    confirm_timeout: Option<TagWrapper<u32>>,
    persist: Option<TagWrapper<String>>,
    #[serde(rename = "persist-id")]
    persist_id: Option<TagWrapper<String>>,
}

pub type CommitResponse = SimpleResponse;
