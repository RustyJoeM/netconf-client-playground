use serde::Serialize;

use super::{tag_wrapper::TagWrapper, Datastore};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(into = "ConfigWaypointRpc")]
pub enum ConfigWaypoint {
    Datastore(Datastore),
    Url(String),
}

#[derive(Debug, Serialize)]
pub struct ConfigWaypointRpc {
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
