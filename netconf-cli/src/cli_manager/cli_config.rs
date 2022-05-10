use std::str::FromStr;

use anyhow::bail;
use netconf_client::types::Capability;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DumpXmlFormat {
    None,
    Raw,
    Pretty,
}

pub struct CliConfig {
    request_dump_mode: DumpXmlFormat,
    response_dump_mode: DumpXmlFormat,
    client_capabilities: Vec<Capability>,
    // log_level: LogLevel, // TODO
}

impl CliConfig {
    pub fn new() -> Self {
        Self {
            request_dump_mode: DumpXmlFormat::None,
            response_dump_mode: DumpXmlFormat::Pretty,
            client_capabilities: vec![Capability::Base],
        }
    }

    pub fn request_dump_mode(&self) -> DumpXmlFormat {
        self.request_dump_mode
    }

    pub fn set_request_dump_mode(&mut self, mode: DumpXmlFormat) {
        self.request_dump_mode = mode;
    }

    pub fn response_dump_mode(&self) -> DumpXmlFormat {
        self.response_dump_mode
    }

    pub fn set_response_dump_mode(&mut self, mode: DumpXmlFormat) {
        self.response_dump_mode = mode;
    }

    pub fn client_capabilities(&self) -> &[Capability] {
        &self.client_capabilities
    }
}

impl FromStr for DumpXmlFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "raw" => Ok(Self::Raw),
            "pretty" => Ok(Self::Pretty),
            s => bail!("Unsupported dump mode: \"{}\"", s),
        }
    }
}
