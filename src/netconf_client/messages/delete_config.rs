use std::fmt::Debug;

use serde::Serialize;

use crate::netconf_client::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, Datastore, SimpleResponse},
};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "DeleteConfigRequestRpc")]
pub struct DeleteConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub target: Datastore,
}

impl super::NetconfRequest for DeleteConfigRequest {
    fn to_netconf_rpc(&self) -> anyhow::Result<String> {
        let res = quick_xml::se::to_string(&self)?;
        Ok(res)
    }
}

impl From<DeleteConfigRequest> for DeleteConfigRequestRpc {
    fn from(request: DeleteConfigRequest) -> Self {
        DeleteConfigRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            delete_config: DeleteConfigRpc {
                target: request.target.into(),
            },
        }
    }
}

impl DeleteConfigRequest {
    pub fn new(message_id: String, target: Datastore) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            target,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct DeleteConfigRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "Delete-config")]
    delete_config: DeleteConfigRpc,
}

#[derive(Debug, Serialize)]
#[serde(rename = "Delete-config")]
struct DeleteConfigRpc {
    target: TagWrapper<Datastore>,
}

pub type DeleteConfigResponse = SimpleResponse;
