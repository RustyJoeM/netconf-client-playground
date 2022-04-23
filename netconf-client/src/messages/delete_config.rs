use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{ConfigWaypoint, ConfigWaypointRpc, SimpleResponse},
};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "DeleteConfigRequestRpc")]
pub struct DeleteConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub target: ConfigWaypoint,
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
    pub fn new(message_id: String, target: ConfigWaypoint) -> Self {
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
    #[serde(rename = "delete-config")]
    delete_config: DeleteConfigRpc,
}

#[derive(Debug, Serialize)]
#[serde(rename = "delete-config")]
struct DeleteConfigRpc {
    target: ConfigWaypointRpc,
}

pub type DeleteConfigResponse = SimpleResponse;
