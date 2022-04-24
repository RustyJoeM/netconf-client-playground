use serde::Serialize;

use super::{tag_wrapper::TagWrapper, Datastore};

/// Artificial unifying enumeration of NETCONF datastore and URL string.
///
/// Used to identify a source/target of `:url` NETCONF capability enabled operations.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(into = "ConfigWaypointRpc")]
pub enum ConfigWaypoint {
    Datastore(Datastore),
    Url(String),
}

/// Private representation of [ConfigWaypoint] used for RPC serialization.
#[derive(Debug, Serialize)]
pub(crate) struct ConfigWaypointRpc {
    datastore: Option<Datastore>,
    url: Option<TagWrapper<String>>,
}

impl From<ConfigWaypoint> for ConfigWaypointRpc {
    fn from(wp: ConfigWaypoint) -> Self {
        match wp {
            ConfigWaypoint::Datastore(datastore) => Self {
                datastore: Some(datastore),
                url: None,
            },
            ConfigWaypoint::Url(url) => Self {
                datastore: None,
                url: Some(TagWrapper::new(url)),
            },
        }
    }
}
