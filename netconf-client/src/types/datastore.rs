use std::str::FromStr;

use serde::Serialize;

/// A configuration datastore identifier.
///
/// Includes standardized RFC defined [`Datastore::Running`], [`Datastore::Candidate`], etc.,
/// as well as `[Datastore::Other] variant to allow custom / non-standardized or implementation specific datastores.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Datastore {
    /// Standard "running" datastore of the actual device configuration.
    Running,
    /// A datastore with pending transaction changes, waiting to be committed/discarded.
    /// Usage of this datastore is typically conditioned with server side support of `:candidate` capability.
    Candidate,
    /// Startup configuration loaded by the device when it boots.
    /// Usage of this datastore is typically conditioned with server side support of `:startup` capability.
    Startup,
    /// A configuration datastore holding the complete intended configuration of the device. (RFC - 8342, [section 5.1.4](https://datatracker.ietf.org/doc/html/rfc8342#section-5.1.4))
    Intended,
    /// A datastore holding the complete operational state of the device. (RFC - 8342, [section 5.3](https://datatracker.ietf.org/doc/html/rfc8342#section-5.3))
    Operational,
    /// A "fallback" datastore for currently non-standardized or lib-unsupported datastores.
    ///
    /// (NOT for URL string - there is standalone type for targets/sources of URL origin - [`crate::types::ConfigWaypoint`])
    Other(String),
}

impl std::fmt::Display for Datastore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = match self {
            Datastore::Running => "running",
            Datastore::Candidate => "candidate",
            Datastore::Startup => "startup",
            Datastore::Intended => "intended",
            Datastore::Operational => "operational",
            Datastore::Other(other) => other,
        };
        f.write_str(ptr)
    }
}

impl FromStr for Datastore {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "running" => Datastore::Running,
            "candidate" => Datastore::Candidate,
            "startup" => Datastore::Startup,
            "intended" => Datastore::Intended,
            "operational" => Datastore::Operational,
            s => Datastore::Other(s.to_string()),
        })
    }
}

impl Datastore {
    /// Provides slice of all the standardized datastore's names.
    // TODO - make capability conditioned?
    pub fn values() -> Vec<&'static str> {
        vec!["running", "candidate", "startup", "intended", "operational"]
    }
}
