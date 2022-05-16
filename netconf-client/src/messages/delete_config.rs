use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::validate_waypoint_url,
    types::{ConfigWaypoint, ConfigWaypointRpc, SimpleResponse},
};

use super::NetconfRequest;

#[derive(Debug, Serialize, Clone)]
#[serde(into = "DeleteConfigRequestRpc")]
pub struct DeleteConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub target: ConfigWaypoint,
}

impl NetconfRequest for DeleteConfigRequest {
    type Response = DeleteConfigResponse;

    fn validate_request(
        &self,
        server_capabilities: &[crate::types::Capability],
    ) -> anyhow::Result<()> {
        validate_waypoint_url(&self.target, server_capabilities)?;
        Ok(())
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
