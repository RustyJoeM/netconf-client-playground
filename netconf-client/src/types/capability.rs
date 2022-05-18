use std::fmt::Display;

use anyhow::bail;

/// A NETCONF capabilities as defined in RFC 6241 - [section 10.4](https://datatracker.ietf.org/doc/html/rfc6241#section-10.4)
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Capability {
    /// `:base-1.0` capability for NETCONF 1.0 plain message exchange
    Base,
    /// `:base-1.1` capability for NETCONF 1.1 chunked framing message exchange
    Base11,
    /// `:writable-running` capability - [RFC 6241 - section 8.2](https://datatracker.ietf.org/doc/html/rfc6241#section-8.2)
    WritableRunning,
    /// `:candidate` capability - [RFC 6241 - section 8.3](https://datatracker.ietf.org/doc/html/rfc6241#section-8.3)
    Candidate,
    /// `:confirmed-commit` capability - [RFC 6241 - section 8.4](https://datatracker.ietf.org/doc/html/rfc6241#section-8.4)
    ConfirmedCommit,
    /// `:rollback-on-error` capability - [RFC 6241 - section 8.5](https://datatracker.ietf.org/doc/html/rfc6241#section-8.5)
    RollbackOnError,
    /// `:validate` capability - [RFC 6241 - section 8.6](https://datatracker.ietf.org/doc/html/rfc6241#section-8.6)
    Validate,
    /// `:startup` capability - [RFC 6241 - section 8.7](https://datatracker.ietf.org/doc/html/rfc6241#section-8.7)
    Startup,
    /// `:url` capability - [RFC 6241 - section 8.8](https://datatracker.ietf.org/doc/html/rfc6241#section-8.8)
    Url(Vec<String>),
    /// `:xpath` capability - [RFC 6241 - section 8.9](https://datatracker.ietf.org/doc/html/rfc6241#section-8.9)
    XPath,
    /// `:validate-1.1` capability for extended <validate> functionality
    Validate11,
    /// Fallback non-standardized or un-implemented capability with full URN as argument.
    Other(String),
}

impl Capability {
    /// Get the URN of the capability defined by RFC.
    pub fn get_urn(&self) -> String {
        match self {
            Capability::Base => "urn:ietf:params:netconf:base:1.0".to_string(),
            Capability::Base11 => "urn:ietf:params:netconf:base:1.1".to_string(),
            Capability::Candidate => "urn:ietf:params:netconf:capability:candidate:1.0".to_string(),
            Capability::ConfirmedCommit => {
                "urn:ietf:params:netconf:capability:confirmed-commit:1.1".to_string()
            }
            Capability::RollbackOnError => {
                "urn:ietf:params:netconf:capability:rollback-on-error:1.0".to_string()
            }
            Capability::Startup => "urn:ietf:params:netconf:capability:startup:1.0".to_string(),
            Capability::Validate => "urn:ietf:params:netconf:capability:validate:1.0".to_string(),
            Capability::Validate11 => "urn:ietf:params:netconf:capability:validate:1.1".to_string(),
            Capability::WritableRunning => {
                "urn:ietf:params:netconf:capability:writable-running:1.0".to_string()
            }
            Capability::XPath => "urn:ietf:params:netconf:capability:xpath:1.0".to_string(),
            Capability::Url(schemes) => format!(
                "urn:ietf:params:netconf:capability:url:1.0?scheme={{{}}}",
                schemes.join(",")
            ),
            Capability::Other(s) => s.clone(),
        }
    }
    // urn:ietf:params:netconf:capability:url:1.0?scheme={name,...}

    /// Construct Capability instance from URN text string.
    pub fn from_urn(urn: &str) -> anyhow::Result<Self> {
        let instance: Capability = match urn {
            "urn:ietf:params:netconf:base:1.0" => Self::Base,
            "urn:ietf:params:netconf:base:1.1" => Self::Base11,
            "urn:ietf:params:netconf:capability:candidate:1.0" => Self::Candidate,
            "urn:ietf:params:netconf:capability:confirmed-commit:1.1" => Self::ConfirmedCommit,
            "urn:ietf:params:netconf:capability:rollback-on-error:1.0" => Self::RollbackOnError,
            "urn:ietf:params:netconf:capability:startup:1.0" => Self::Startup,
            "urn:ietf:params:netconf:capability:writable-running:1.0" => Self::WritableRunning,
            "urn:ietf:params:netconf:capability:xpath:1.0" => Self::XPath,
            "urn:ietf:params:netconf:capability:validate:1.0" => Self::Validate,
            "urn:ietf:params:netconf:capability:validate:1.1" => Self::Validate11,
            s => {
                // Format for URL schemes - https://datatracker.ietf.org/doc/html/rfc6241#section-8.8.3
                if s.starts_with("urn:ietf:params:netconf:capability:url:1.0") {
                    if let Some(s) = s.split("scheme=").nth(1) {
                        let schemes = s.split(',').map(|x| x.to_string()).collect();
                        Self::Url(schemes)
                    } else {
                        bail!("Unsupported URL capability string - unable to parse schemes!");
                    }
                } else {
                    Self::Other(s.to_string())
                }
            }
        };
        Ok(instance)
    }
}

impl Display for Capability {
    // TODO - what about schemes for Url capability?
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Capability::Base => ":base:1.0",
            Capability::Base11 => ":base:1.1",
            Capability::Candidate => ":candidate:1.0",
            Capability::ConfirmedCommit => ":confirmed-commit:1.1",
            Capability::RollbackOnError => ":rollback-on-error:1.0",
            Capability::Startup => ":startup:1.0",
            Capability::Url(_) => ":url:1.0",
            Capability::Validate => ":validate:1.0",
            Capability::Validate11 => ":validate:1.1",
            Capability::WritableRunning => ":writable-running:1.0",
            Capability::XPath => ":xpath:1.0",
            Capability::Other(other) => other,
        };
        write!(f, "{}", s)
    }
}
