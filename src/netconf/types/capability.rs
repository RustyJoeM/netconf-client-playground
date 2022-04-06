use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Enumeration of NETCONF capabilities defined in RFC6241.
/// (https://datatracker.ietf.org/doc/html/rfc6241#section-10.4)
#[derive(Debug, PartialEq, Clone)]
pub enum Capability {
    WritableRunning,
    Candidate,
    RollbackOnError,
    Startup,
    Url,
    XPath,
    Base10,
    Base11,
    ConfirmedCommit,
    Validate,
    // for other non standardized ones...
    Other(String),
}

impl Capability {
    /// Get the URN of the capability defined by RFC.
    pub fn get_urn(&self) -> &str {
        match self {
            Capability::WritableRunning => {
                "urn:ietf:params:netconf:capability:writable-running:1.0"
            }
            Capability::Candidate => "urn:ietf:params:netconf:capability:candidate:1.0",
            Capability::RollbackOnError => {
                "urn:ietf:params:netconf:capability:rollback-on-error:1.0"
            }
            Capability::Startup => "urn:ietf:params:netconf:capability:startup:1.0",
            Capability::Url => "urn:ietf:params:netconf:capability:url:1.0",
            Capability::XPath => "urn:ietf:params:netconf:capability:xpath:1.0",
            Capability::Base10 => "urn:ietf:params:netconf:base:1.0",
            Capability::Base11 => "urn:ietf:params:netconf:base:1.1",
            Capability::ConfirmedCommit => {
                "urn:ietf:params:netconf:capability:confirmed-commit:1.1"
            }
            Capability::Validate => "urn:ietf:params:netconf:capability:validate:1.1",
            Capability::Other(s) => s,
        }
    }

    /// Construct Capability instance from URN text string.
    pub fn from_urn(urn: &str) -> Self {
        match urn {
            "urn:ietf:params:netconf:capability:writable-running:1.0" => Self::WritableRunning,
            "urn:ietf:params:netconf:capability:candidate:1.0" => Self::Candidate,
            "urn:ietf:params:netconf:capability:rollback-on-error:1.0" => Self::RollbackOnError,
            "urn:ietf:params:netconf:capability:startup:1.0" => Self::Startup,
            "urn:ietf:params:netconf:capability:url:1.0" => Self::Url,
            "urn:ietf:params:netconf:capability:xpath:1.0" => Self::XPath,
            "urn:ietf:params:netconf:base:1.0" => Self::Base10,
            "urn:ietf:params:netconf:base:1.1" => Self::Base11,
            "urn:ietf:params:netconf:capability:confirmed-commit:1.1" => Self::ConfirmedCommit,
            "urn:ietf:params:netconf:capability:validate:1.1" => Self::Validate,
            s => Self::Other(s.to_string()),
        }
    }
}

impl Serialize for Capability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.get_urn())
    }
}

impl<'de> Deserialize<'de> for Capability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Capability::from_urn(&s))
    }
}
