use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::{validate_datastore_capability, validate_url},
    types::{tag_wrapper::TagWrapper, Capability, Datastore, SimpleResponse},
};

use super::NetconfRequest;

/// Source of the configuration to be validated by [`ValidateRequest`].
#[derive(Debug, Clone)]
pub enum ValidateSource {
    /// Plain whole datastore (e.g. "candidate").
    Datastore(Datastore),
    /// Explicit configuration dump in XML format.
    Config(String),
    /// URL string for :url capability enabled client/server.
    Url(String),
}

/// Representation of NETCONF `<validate>` operation request.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "ValidateRequestRpc")]
pub struct ValidateRequest {
    message_id: String,
    source: ValidateSource,
}

impl ValidateRequest {
    /// Creates new instance of NETCONF `<validate>` operation.
    pub fn new(message_id: String, source: ValidateSource) -> Self {
        Self { message_id, source }
    }
}

impl NetconfRequest for ValidateRequest {
    type Response = ValidateResponse;

    fn validate_request(
        &self,
        server_capabilities: &[crate::types::Capability],
    ) -> anyhow::Result<()> {
        match &self.source {
            ValidateSource::Datastore(datastore) => {
                validate_datastore_capability(
                    datastore,
                    &Datastore::Candidate,
                    &Capability::Candidate,
                    server_capabilities,
                )?;
            }
            ValidateSource::Config(_) => {} // TODO - might check for valid/paired XML tags or something?
            ValidateSource::Url(url) => validate_url(url, server_capabilities)?,
        };

        Ok(())
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct ValidateRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    validate: ValidateRpc,
}

#[derive(Debug, Serialize)]
struct ValidateRpc {
    source: SourceRpc,
}

#[derive(Debug, Serialize)]
struct SourceRpc {
    config: Option<TagWrapper<String>>,
    #[serde(rename = "$value")]
    datastore: Option<Datastore>,
    url: Option<TagWrapper<String>>,
}

impl From<ValidateRequest> for ValidateRequestRpc {
    fn from(request: ValidateRequest) -> Self {
        let source = match &request.source {
            ValidateSource::Datastore(datastore) => SourceRpc {
                datastore: Some(datastore.clone()),
                config: None,
                url: None,
            },
            ValidateSource::Config(config) => SourceRpc {
                datastore: None,
                config: Some(TagWrapper::new(config.clone())),
                url: None,
            },
            ValidateSource::Url(url) => SourceRpc {
                datastore: None,
                config: None,
                url: Some(TagWrapper::new(url.clone())),
            },
        };

        ValidateRequestRpc {
            xmlns: XMLNS.to_string(),
            message_id: request.message_id,
            validate: ValidateRpc { source },
        }
    }
}

/// Representation of a server response to [`ValidateRequest`].
pub type ValidateResponse = SimpleResponse;
