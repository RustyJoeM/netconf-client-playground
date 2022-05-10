use clap::{Parser, Subcommand};

use super::{
    cli_config_commands::CliConfigCommand, netconf_commands::NetconfCommand,
    show_commands::ShowCommand,
};
use crate::cli_manager::{CliManagerCommandApi, DumpXmlFormat};

use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(multicall(true), disable_help_flag(true))]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub struct RootArgs {
    #[clap(subcommand)]
    pub command: RootCommand,
}

#[derive(Subcommand, Debug)]
pub enum RootCommand {
    /// Modify the CLI behavior by applying various settings.
    #[clap(subcommand)]
    Config(CliConfigCommand),
    /// Dump the NETCONF request only, do not send to server
    #[clap(subcommand)]
    Dump(NetconfCommand),
    /// Raw NETCONF request dispatched to target server with subsequent response dump.
    #[clap(subcommand)]
    Send(NetconfCommand),
    /// Display various aspects of running CLI/NETCONF session, etc.
    #[clap(subcommand)]
    Show(ShowCommand),
}

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
            RootCommand::Config(command) => command.handle(cli_api),
            RootCommand::Dump(command) => {
                let _ = command.dump_command_xml(DumpXmlFormat::Pretty, "XML dump only:", "dump");
                Ok(())
            }
            RootCommand::Send(command) => command.handle(cli_api),
            RootCommand::Show(command) => command.handle(cli_api),
        }
    }

    // pub fn command_names() -> Vec<&'static str> {
    //     vec!["config", "dump", "send", "help"]
    // }
}
