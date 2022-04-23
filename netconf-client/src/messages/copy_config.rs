use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{ConfigWaypoint, ConfigWaypointRpc, SimpleResponse},
};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "CopyConfigRequestRpc")]
pub struct CopyConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub target: ConfigWaypoint,
    pub source: ConfigWaypoint,
}

impl super::NetconfRequest for CopyConfigRequest {
    fn to_netconf_rpc(&self) -> anyhow::Result<String> {
        let res = quick_xml::se::to_string(&self)?;
        Ok(res)
    }
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
    pub fn new(message_id: String, target: ConfigWaypoint, source: ConfigWaypoint) -> Self {
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
    target: ConfigWaypointRpc,
    source: ConfigWaypointRpc,
}

pub type CopyConfigResponse = SimpleResponse;
