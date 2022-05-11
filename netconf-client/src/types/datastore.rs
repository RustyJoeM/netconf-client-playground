use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Datastore {
    Running,
    Candidate,
    Startup,
    Other(String), // TODO - fix unknown datastore ids & problem with extra <other> wrapper in serialization
}

impl std::fmt::Display for Datastore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = match self {
            Datastore::Running => "running",
            Datastore::Candidate => "candidate",
            Datastore::Startup => "startup",
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
            s => Datastore::Other(s.to_string()),
        })
    }
}

impl Datastore {
    pub fn values() -> Vec<&'static str> {
        vec!["running", "candidate", "startup"]
    }
}
