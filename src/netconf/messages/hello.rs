use serde::{Deserialize, Serialize};

use crate::netconf::{capability::Capability, common::XMLNS};

/// Client \<hello\> request to be sent to NETCONF server when initiating the connection.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "HelloRequestRpc")]
pub struct HelloRequest {
    pub capabilities: Vec<Capability>,
}

impl HelloRequest {
    /// Create instance of RPC for \<hello\> request.
    /// Pass vector of supported client capabilities,
    /// or empty vector for default Base capability only.
    pub fn new(capabilities: Vec<Capability>) -> Self {
        Self {
            capabilities: match capabilities.is_empty() {
                true => vec![Capability::Base10],
                false => capabilities,
            },
        }
    }
}

/// Private representation for serialization of \<hello\> request.
#[derive(Debug, Serialize)]
#[serde(rename = "hello")]
struct HelloRequestRpc {
    xmlns: String,
    capabilities: CapabilitiesRpc,
}

#[derive(Debug, Serialize, Deserialize)]
struct CapabilitiesRpc {
    #[serde(rename = "capability")]
    items: Vec<CapabilityRpc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CapabilityRpc {
    #[serde(rename = "$value")]
    item: Capability,
}

impl From<HelloRequest> for HelloRequestRpc {
    fn from(request: HelloRequest) -> Self {
        let capabilities = CapabilitiesRpc {
            items: request
                .capabilities
                .iter()
                .map(|cap| CapabilityRpc { item: cap.clone() })
                .collect(),
        };
        HelloRequestRpc {
            xmlns: XMLNS.to_string(),
            capabilities,
        }
    }
}

/// Initial \<hello\> response sent by NETCONF server when initiation the session.
#[derive(Debug, Deserialize)]
#[serde(from = "HelloResponseRpc")]
pub struct HelloResponse {
    #[serde(rename = "session-id")]
    pub session_id: u32,
    pub capabilities: Vec<Capability>,
}

/// Private representation for de-serialization of \<hello\> request.
#[derive(Debug, Deserialize)]
struct HelloResponseRpc {
    #[serde(rename = "xmlns")]
    _xmlns: String,
    #[serde(rename = "session-id")]
    session_id: u32,
    capabilities: CapabilitiesRpc,
}

impl From<HelloResponseRpc> for HelloResponse {
    fn from(rpc: HelloResponseRpc) -> Self {
        HelloResponse {
            session_id: rpc.session_id,
            capabilities: rpc
                .capabilities
                .items
                .iter()
                .map(|cap| cap.item.clone())
                .collect(),
        }
    }
}
