use std::net::IpAddr;

use clap::{Parser, Subcommand};
use netconf_client::types::Datastore;

const HELP_TEMPLATE: &str = "{all-args}";

#[derive(Parser, Debug)]
#[clap(
    no_binary_name(true),
    // disable_help_subcommand(true),
    // disable_help_flag(true),
    // help_template(HELP_TEMPLATE)
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
#[clap(no_binary_name(true))]
pub enum Command {
    #[clap(
        about("Clear the whole previously saved history buffer."),
        disable_help_flag(true),
        arg_required_else_help = false
    )]
    HistoryClear {},
    #[clap(
    // display_order(0),
    about("<hello> request to initiate connection with NETCONF server"),
    disable_help_flag(true),
    arg_required_else_help = false
    )]
    Hello {
        #[clap(long)]
        address: IpAddr,
        #[clap(long)]
        port: u16,
        #[clap(long)]
        user: String,
        #[clap(long)]
        password: String,
    },
    Lock {
        #[clap(possible_values(["running", "candidate"]))]
        target: Datastore,
    },
    Unlock {
        #[clap(possible_values(["running", "candidate"]))]
        target: Datastore,
    },
    #[clap(about("Dispatch <close-session> request for currently opened NETCONF session."))]
    CloseSession {},
    // #[clap(
    //     display_order(1),
    //     about("Maze with a mask defined by input text file"),
    //     disable_help_flag(true),
    //     arg_required_else_help = true
    // )]
    // Masked {
    //     #[clap(short, long, help("Path to mask text file"), display_order(0))]
    //     path: String,
    // }
}
