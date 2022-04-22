use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Datastore {
    Running,
    Candidate,
    Other(String), // TODO - fix unknown datastore ids & problem with extra <other> wrapper in serialization
}

impl std::fmt::Display for Datastore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = match self {
            Datastore::Running => "running",
            Datastore::Candidate => "candidate",
            Datastore::Other(other) => other,
        };
        f.write_str(ptr)
    }
}
