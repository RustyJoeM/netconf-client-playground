use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Datastore {
    Running,
    Candidate,
    Other(String), // TODO - fix unknown datastore ids & problem with extra <other> wrapper in serialization
}

// impl Serialize for Datastore {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = match self {
//             Datastore::Other(s) => s.as_str(),
//             Datastore::Running => "running",
//             Datastore::Candidate => "candidate",
//         };
//         let x = format!("<{}/>", s);
//         serializer.serialize_str(&x)
//     }
// }
