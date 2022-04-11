use serde::{Deserialize, Serialize};

use crate::netconf::{common::XMLNS, types::Capability};

/// Client \<hello\> request to be sent to NETCONF server when initiating the connection.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "HelloRequestRpc")]
pub struct HelloRequest {
    /// List of capabilities supported by client initiating the NETCONF connection.
    capabilities: Vec<Capability>,
}

impl HelloRequest {
    /// Create instance of \<hello\> request.
    /// Pass vector of supported client capabilities as an input,
    /// or empty vector for default "base-1.0" capability only.
    pub fn new(capabilities: Vec<Capability>) -> Self {
        Self {
            capabilities: match capabilities.is_empty() {
                true => vec![Capability::Base],
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
    item: String,
}

impl From<HelloRequest> for HelloRequestRpc {
    /// Automatically convert `HelloRequest` info/from its RPC representation struct used in serialization.
    fn from(request: HelloRequest) -> Self {
        let capabilities = CapabilitiesRpc {
            items: request
                .capabilities
                .iter()
                .map(|cap| CapabilityRpc {
                    item: cap.get_urn().to_string(),
                })
                .collect(),
        };
        HelloRequestRpc {
            xmlns: XMLNS.to_string(),
            capabilities,
        }
    }
}

/// Initial \<hello\> response sent by NETCONF server.
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
    /// Automatically convert `HelloResponse` info/from its RPC representation struct used in serialization.
    fn from(rpc: HelloResponseRpc) -> Self {
        HelloResponse {
            session_id: rpc.session_id,
            capabilities: rpc
                .capabilities
                .items
                .iter()
                .map(|cap| Capability::from_urn(&cap.item))
                .collect(),
        }
    }
}
