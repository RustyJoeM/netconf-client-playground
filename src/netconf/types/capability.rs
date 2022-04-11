/// Enumeration of NETCONF capabilities defined in RFC6241.
/// (https://datatracker.ietf.org/doc/html/rfc6241#section-10.4)
#[derive(Debug, Clone, PartialEq)]
pub enum Capability {
    Base,
    Base11,
    Candidate,
    ConfirmedCommit,
    RollbackOnError,
    Startup,
    Url,
    Validate,
    WritableRunning,
    XPath,
    // for other non standardized ones...
    Other(String),
}

impl Capability {
    /// Get the URN of the capability defined by RFC.
    pub fn get_urn(&self) -> &str {
        match self {
            Capability::Base => "urn:ietf:params:netconf:base:1.0",
            Capability::Base11 => "urn:ietf:params:netconf:base:1.1",
            Capability::Candidate => "urn:ietf:params:netconf:capability:candidate:1.0",
            Capability::ConfirmedCommit => {
                "urn:ietf:params:netconf:capability:confirmed-commit:1.1"
            }
            Capability::RollbackOnError => {
                "urn:ietf:params:netconf:capability:rollback-on-error:1.0"
            }
            Capability::Startup => "urn:ietf:params:netconf:capability:startup:1.0",
            Capability::Url => "urn:ietf:params:netconf:capability:url:1.0",
            Capability::Validate => "urn:ietf:params:netconf:capability:validate:1.1",
            Capability::WritableRunning => {
                "urn:ietf:params:netconf:capability:writable-running:1.0"
            }
            Capability::XPath => "urn:ietf:params:netconf:capability:xpath:1.0",
            Capability::Other(s) => s,
        }
    }

    /// Construct Capability instance from URN text string.
    pub fn from_urn(urn: &str) -> Self {
        match urn {
            "urn:ietf:params:netconf:base:1.0" => Self::Base,
            "urn:ietf:params:netconf:base:1.1" => Self::Base11,
            "urn:ietf:params:netconf:capability:candidate:1.0" => Self::Candidate,
            "urn:ietf:params:netconf:capability:confirmed-commit:1.1" => Self::ConfirmedCommit,
            "urn:ietf:params:netconf:capability:rollback-on-error:1.0" => Self::RollbackOnError,
            "urn:ietf:params:netconf:capability:startup:1.0" => Self::Startup,
            "urn:ietf:params:netconf:capability:url:1.0" => Self::Url,
            "urn:ietf:params:netconf:capability:writable-running:1.0" => Self::WritableRunning,
            "urn:ietf:params:netconf:capability:xpath:1.0" => Self::XPath,
            "urn:ietf:params:netconf:capability:validate:1.1" => Self::Validate,
            s => Self::Other(s.to_string()),
        }
    }
}
