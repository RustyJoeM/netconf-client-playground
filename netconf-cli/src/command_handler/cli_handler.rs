use anyhow::Result;
use clap::Subcommand;

use crate::cli_manager::{CliManagerCommandApi, DumpXmlFormat};

#[derive(Subcommand, Debug)]
pub enum CliConfigCommand {
    /// Perform dump of XML to console when exchanging messages with NETCONF server.
    #[clap(no_binary_name(true))]
    XmlDump {
        #[clap(long)]
        request: DumpXmlFormat,
        #[clap(long)]
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
