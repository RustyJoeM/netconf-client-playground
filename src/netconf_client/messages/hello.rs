use serde::{Deserialize, Serialize};

use crate::netconf_client::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, Capability},
};

/// Client \<hello\> request to be sent to NETCONF server when initiating the connection.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "HelloRequestRpc")]
pub struct HelloRequest {
    /// List of capabilities supported by client initiating the NETCONF connection.
    capabilities: Vec<Capability>,
}

impl HelloRequest {
    /// Create instance of \<hello\> request.
    /// Pass vector of supported client capabilities as an input.
    pub fn new(capabilities: Vec<Capability>) -> Self {
        Self { capabilities }
    }
}

impl super::NetconfRequest for HelloRequest {
    fn to_netconf_rpc(&self) -> anyhow::Result<String> {
        let res = quick_xml::se::to_string(self)?;
        Ok(res)
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
    items: Vec<TagWrapper<String>>,
}

impl From<HelloRequest> for HelloRequestRpc {
    /// Automatically convert `HelloRequest` info/from its RPC representation struct used in serialization.
    fn from(request: HelloRequest) -> Self {
        let capabilities = CapabilitiesRpc {
            items: request
                .capabilities
                .iter()
                .map(|cap| cap.get_urn().into())
                .collect(),
        };
        HelloRequestRpc {
            xmlns: XMLNS.to_string(),
            capabilities,
        }
    }
}

// TODO - error case?

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
                .map(|cap| Capability::from_urn(&cap.item).unwrap())
                .collect(),
        }
    }
}

impl super::NetconfResponse for HelloResponse {
    fn from_netconf_rpc(s: String) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let res = quick_xml::de::from_str(&s)?;
        Ok(res)
    }
}
