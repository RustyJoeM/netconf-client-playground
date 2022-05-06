use clap::{Parser, Subcommand};

use super::{cli_handler::CliConfigCommand, netconf_handler::NetconfCommand};

#[derive(Parser, Debug)]
#[clap(multicall(true), disable_help_flag(true))]
pub struct RootArgs {
    #[clap(subcommand)]
    pub command: RootCommand,
}

#[derive(Subcommand, Debug)]
pub enum RootCommand {
    /// Dump the NETCONF request only, do not send to server
    #[clap(subcommand, display_order(1))]
    Dump(NetconfCommand),
    /// Raw NETCONF request dispatched to target server with subsequent response dump.
    #[clap(subcommand, display_order(1))]
    Send(NetconfCommand),
    /// Modify the CLI behavior by applying various settings.
    #[clap(subcommand, display_order(1))]
    Config(CliConfigCommand),
    // #[clap(
    //     about("Clear the whole previously saved history buffer."),
    //     disable_help_flag(true),
    //     arg_required_else_help = false
    // )]
}
