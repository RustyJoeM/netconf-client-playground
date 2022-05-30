use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::{validate_datastore_capability, validate_url},
    types::{Capability, ConfigWaypoint, ConfigWaypointRpc, Datastore, SimpleResponse},
};

use super::NetconfRequest;

/// Representation of NETCONF `<copy-config>` operation request.
/// Create or replace an entire configuration datastore with the contents of another complete configuration datastore.
#[derive(Debug, Serialize, Clone)]
#[serde(into = "CopyConfigRequestRpc")]
pub struct CopyConfigRequest {
    message_id: String,
    xmlns: String,
    target: ConfigWaypoint,
    source: ConfigWaypoint,
}

impl NetconfRequest for CopyConfigRequest {
    type Response = CopyConfigResponse;

    fn validate_request(
        &self,
        server_capabilities: &[crate::types::Capability],
    ) -> anyhow::Result<()> {
        match &self.target {
            ConfigWaypoint::Datastore(datastore) => {
                validate_datastore_capability(
                    datastore,
                    &Datastore::Running,
                    &Capability::WritableRunning,
                    server_capabilities,
                )?;
                validate_datastore_capability(
                    datastore,
                    &Datastore::Candidate,
                    &Capability::Candidate,
                    server_capabilities,
                )?;
            }
            ConfigWaypoint::Url(url) => {
                validate_url(url, server_capabilities)?;
            }
        };

        match &self.source {
            ConfigWaypoint::Datastore(datastore) => {
                validate_datastore_capability(
                    datastore,
                    &Datastore::Candidate,
                    &Capability::Candidate,
                    server_capabilities,
                )?;
            }
            ConfigWaypoint::Url(url) => {
                validate_url(url, server_capabilities)?;
            }
        };

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
    /// Creates new instance of NETCONF `<copy-config>` operation request.
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

/// Representation of a server response to [`CopyConfigRequest`].
pub type CopyConfigResponse = SimpleResponse;
