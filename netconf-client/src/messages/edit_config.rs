use std::fmt::Debug;

use anyhow::Result;
use quick_xml::se::to_string;
use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, Datastore, SimpleResponse},
};

/// TODO - operation for \<edit-config\> elements, but not used due to nested generic XML here...
pub enum Operation {
    Merge,
    Replace,
    Create,
    Delete,
    Remove,
}

#[derive(Debug, Serialize, Clone)]
pub enum DefaultOperation {
    Merge,
    Replace,
    None,
}

#[derive(Debug, Serialize, Clone)]
pub enum TestOption {
    TestThenSet,
    Set,
    TestOnly,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum ErrorOption {
    StopOnError,
    ContinueOnError,
    RollbackOnError,
}

#[derive(Debug, Serialize, Clone)]
#[serde(into = "EditConfigRequestRpc")]
pub struct EditConfigRequest {
    pub message_id: String,
    pub xmlns: String,
    pub params: EditConfigParams,
}

#[derive(Debug, Clone)]
pub enum EditConfigContent {
    Config(String),
    Url(String),
}

#[derive(Debug, Clone)]
pub struct EditConfigParams {
    pub target: Datastore,
    pub default_operation: Option<DefaultOperation>,
    pub test_option: Option<TestOption>,
    pub error_option: Option<ErrorOption>,
    pub config: EditConfigContent,
}

impl From<EditConfigRequest> for EditConfigRequestRpc {
    fn from(request: EditConfigRequest) -> Self {
        EditConfigRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            edit_config: request.params.into(),
        }
    }
}

impl From<EditConfigParams> for EditConfigRpc {
    fn from(params: EditConfigParams) -> Self {
        let config = match &params.config {
            EditConfigContent::Config(config) => Some(TagWrapper::new(config.to_owned())),
            EditConfigContent::Url(_) => None,
        };
        let url = match &params.config {
            EditConfigContent::Config(_) => None,
            EditConfigContent::Url(url) => Some(TagWrapper::new(url.to_owned())),
        };
        Self {
            target: TagWrapper::new(params.target),
            default_operation: params.default_operation.map(TagWrapper::new),
            test_option: params.test_option.map(TagWrapper::new),
            error_option: params.error_option.map(TagWrapper::new),
            config,
            url,
        }
    }
}

impl EditConfigRequest {
    pub fn new(message_id: String, params: EditConfigParams) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            params,
        }
    }
}

impl super::NetconfRequest for EditConfigRequest {
    fn to_netconf_rpc(&self) -> Result<std::string::String, anyhow::Error> {
        const TOKEN: &str = "MAGIC_TOKEN";
        let mut params = self.params.clone();

        // reset <config> / <url> contents for automatic serialization to a TOKEN to be replaced later
        let config_backup = params.config;
        let s = TOKEN.to_string();
        params.config = match &config_backup {
            EditConfigContent::Config(_) => EditConfigContent::Config(s),
            EditConfigContent::Url(_) => EditConfigContent::Url(s),
        };

        // serialize RPC without <config> / <url> data
        let instance = Self::new(self.message_id.clone(), params);
        let mut instance_str = to_string(&instance)?;
        // replace back the original filter data (auto would have escaped tags to html &lt; / &gt;)
        let s = match &config_backup {
            EditConfigContent::Config(config) => config,
            EditConfigContent::Url(url) => url,
        };
        instance_str = instance_str.replace(TOKEN, s);
        Ok(instance_str)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct EditConfigRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "edit-config")]
    edit_config: EditConfigRpc,
}

#[derive(Debug, Serialize)]
#[serde(rename = "edit-config")]
struct EditConfigRpc {
    target: TagWrapper<Datastore>,
    #[serde(rename = "default-operation")]
    default_operation: Option<TagWrapper<DefaultOperation>>,
    #[serde(rename = "test-option")]
    test_option: Option<TagWrapper<TestOption>>,
    #[serde(rename = "error-option")]
    error_option: Option<TagWrapper<ErrorOption>>,
    config: Option<TagWrapper<String>>,
    url: Option<TagWrapper<String>>,
}

#[derive(Debug, Serialize)]
struct ConfigRpc {
    #[serde(rename = "$value")]
    item: String,
}

pub type EditConfigResponse = SimpleResponse;
