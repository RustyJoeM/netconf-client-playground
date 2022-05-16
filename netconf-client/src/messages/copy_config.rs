use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::{validate_datastore_capability, validate_waypoint_url},
    types::{Capability, ConfigWaypoint, ConfigWaypointRpc, Datastore, SimpleResponse},
};

use super::NetconfRequest;

#[derive(Debug, Serialize, Clone)]
#[serde(into = "CopyConfigRequestRpc")]
pub struct CopyConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub target: ConfigWaypoint,
    pub source: ConfigWaypoint,
}

impl NetconfRequest for CopyConfigRequest {
    type Response = CopyConfigResponse;

    fn validate_request(
        &self,
        server_capabilities: &[crate::types::Capability],
    ) -> anyhow::Result<()> {
        for (check_if_equal_to, needed_capability) in &[
            (Datastore::Running, Capability::WritableRunning),
            (Datastore::Candidate, Capability::Candidate),
        ] {
            if let ConfigWaypoint::Datastore(datastore) = &self.target {
                validate_datastore_capability(
                    datastore,
                    check_if_equal_to,
                    needed_capability,
                    server_capabilities,
                )?;
            }
        }
        validate_waypoint_url(&self.target, server_capabilities)?;
        validate_waypoint_url(&self.source, server_capabilities)?;
        Ok(())
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
