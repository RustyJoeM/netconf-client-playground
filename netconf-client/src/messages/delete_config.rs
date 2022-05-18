use std::fmt::Debug;

use anyhow::bail;
use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::validate_waypoint_url,
    types::{ConfigWaypoint, ConfigWaypointRpc, Datastore, SimpleResponse},
};

use super::NetconfRequest;

/// Representation of NETCONF `<delete-config>` operation request - Delete a configuration datastore.
/// The <running> configuration datastore cannot be deleted.
#[derive(Debug, Serialize, Clone)]
#[serde(into = "DeleteConfigRequestRpc")]
pub struct DeleteConfigRequest {
    message_id: String,
    xmlns: String,
    target: ConfigWaypoint,
}

impl NetconfRequest for DeleteConfigRequest {
    type Response = DeleteConfigResponse;

    fn validate_request(
        &self,
        server_capabilities: &[crate::types::Capability],
    ) -> anyhow::Result<()> {
        if let ConfigWaypoint::Datastore(datastore) = &self.target {
            if datastore == &Datastore::Running {
                bail!("Cannot delete \"running\" datastore");
            }
        }
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
    /// Creates new instance of NETCONF `<delete-config>` operation request.
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

/// Representation of a server response to [`DeleteConfigRequest`].
pub type DeleteConfigResponse = SimpleResponse;
