use std::fmt::Debug;

use anyhow::Result;
use quick_xml::se::to_string;
use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::{
        validate_capability_presence, validate_datastore_capability, validate_url,
    },
    types::{tag_wrapper::TagWrapper, Capability, Datastore, SimpleResponse},
};

use super::{NetconfRequest, ToPrettyXml, ToRawXml};

// TODO - untested - possibly unfinished/incorrect (de)serialization...

/// TODO - operation for `<edit-config>` elements, but not used due to nested generic XML here...
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Merge,
    Replace,
    Create,
    Delete,
    Remove,
}

/// Possible values of the `default-operation` parameter.
#[derive(Debug, Serialize, Copy, Clone, PartialEq)]
pub enum DefaultOperation {
    /// The configuration data in the <config> parameter is
    /// merged with the configuration at the corresponding level in
    /// the target datastore. This is the default behavior.
    Merge,
    /// The configuration data in the <config> parameter
    /// completely replaces the configuration in the target
    /// datastore. This is useful for loading previously saved
    /// configuration data.
    Replace,
    /// The target datastore is unaffected by the configuration
    /// in the <config> parameter, unless and until the incoming
    /// configuration data uses the "operation" attribute to request
    /// a different operation. If the configuration in the <config>
    /// parameter contains data for which there is not a
    /// corresponding level in the target datastore, an <rpc-error>
    /// is returned with an <error-tag> value of data-missing.
    /// Using "none" allows operations like "delete" to avoid
    /// unintentionally creating the parent hierarchy of the element
    /// to be deleted.
    None,
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq)]
pub enum TestOption {
    TestThenSet,
    Set,
    TestOnly,
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum ErrorOption {
    StopOnError,
    ContinueOnError,
    RollbackOnError,
}

/// Representation of NETCONF `<edit-config>` operation request -
#[derive(Debug, Clone)]
pub struct EditConfigRequest {
    message_id: String,
    xmlns: String,
    params: EditConfigParams,
}

/// Payload of the configuration to be edited.
#[derive(Debug, Clone)]
pub enum EditConfigContent {
    /// Raw XML dump of the configuration changes.
    Config(String),
    /// URL of the changes to be performed for `:url` capability enabled servers.
    Url(String),
}

/// Input parameters of the `<edit-config>` operation request.
/// See [RFC 6421 - section 7.2](https://datatracker.ietf.org/doc/html/rfc6241#section-7.2)
#[derive(Debug, Clone)]
pub struct EditConfigParams {
    /// Name of the configuration datastore being edited.
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
    /// Creates new instance of NETCONF `<edit-config>` operation request.
    pub fn new(message_id: String, params: EditConfigParams) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            params,
        }
    }
}

impl ToRawXml for EditConfigRequest {
    fn to_raw_xml(&self) -> Result<std::string::String, anyhow::Error> {
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
        let instance: EditConfigRequestRpc = instance.into();
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

impl ToPrettyXml for EditConfigRequest {}

impl NetconfRequest for EditConfigRequest {
    type Response = EditConfigResponse;

    fn validate_request(&self, server_capabilities: &[crate::types::Capability]) -> Result<()> {
        validate_datastore_capability(
            &self.params.target,
            &Datastore::Candidate,
            &Capability::Candidate,
            server_capabilities,
        )?;
        if let Some(ErrorOption::RollbackOnError) = &self.params.error_option {
            validate_capability_presence(
                &Capability::RollbackOnError,
                server_capabilities,
                " Cannot use \"rollback-on-error\" <error-option> parameter.",
            )?;
        }
        if self.params.test_option.is_some() {
            validate_capability_presence(
                &Capability::Validate11,
                server_capabilities,
                " Cannot use <test-option> parameter.",
            )?;
        }
        if let EditConfigContent::Url(url) = &self.params.config {
            validate_url(url, server_capabilities)?;
        }
        Ok(())
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

/// Representation of a server response to [`EditConfigRequest`].
pub type EditConfigResponse = SimpleResponse;
