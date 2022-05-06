use crate::cli_manager::{CliManagerCommandApi, DumpXmlFormat};

use super::clap_commands::RootCommand;

use anyhow::Result;

pub struct CommandHandler {}

impl CommandHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_command(
        &mut self,
        cli_api: &mut impl CliManagerCommandApi,
        command: &RootCommand,
    ) -> Result<()> {
        match command {
            RootCommand::Dump(command) => {
                let _ = command.dump_command_xml(DumpXmlFormat::Pretty, "XML dump only:", "dump");
                Ok(())
            }
            RootCommand::Send(command) => command.handle(cli_api),
            RootCommand::Config(command) => command.handle(cli_api),
        }
    }
}
