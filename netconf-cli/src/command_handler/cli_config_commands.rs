use anyhow::Result;
use clap::Subcommand;

use crate::cli_manager::{CliManagerCommandApi, DumpXmlFormat};

#[derive(Subcommand, Debug)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub enum CliConfigCommand {
    /// Perform dump of XML to console when exchanging messages with NETCONF server.
    XmlDump {
        #[clap(long, possible_values = DumpXmlFormat::values())]
        request: DumpXmlFormat,
        #[clap(long, possible_values = DumpXmlFormat::values())]
        response: DumpXmlFormat,
    },
}

impl CliConfigCommand {
    pub fn handle(&self, cli_api: &mut impl CliManagerCommandApi) -> Result<()> {
        match self {
            CliConfigCommand::XmlDump { request, response } => {
                cli_api.config_mut().set_request_dump_mode(*request);
                cli_api.config_mut().set_response_dump_mode(*response);
            }
        }
        Ok(())
    }
}
