use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, Datastore, SimpleResponse},
};

#[derive(Debug, Clone)]
pub enum ValidateSource {
    Datastore(Datastore),
    Config(String),
    Url(String),
}

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ValidateRequestRpc")]
pub struct ValidateRequest {
    message_id: String,
    source: ValidateSource,
}

impl ValidateRequest {
    pub fn new(message_id: String, source: ValidateSource) -> Self {
        Self { message_id, source }
    }
}

impl super::NetconfRequest for ValidateRequest {
    type Response = ValidateResponse;

    fn to_netconf_rpc(&self) -> anyhow::Result<String> {
        let res = quick_xml::se::to_string(self)?;
        Ok(res)
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

pub type ValidateResponse = SimpleResponse;
