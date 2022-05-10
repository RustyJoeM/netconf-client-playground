use std::str::FromStr;

use anyhow::{bail, Result};
use clap::Subcommand;
use colored::Colorize;

use crate::{
    cli_manager::CliManagerCommandApi, command_handler::netconf_commands::NO_SESSION_ERROR_STR,
};

#[derive(Subcommand, Debug)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub enum ShowCommand {
    /// Show list of capabilities advertised by NETCONF client/server.
    ///
    Capabilities {
        #[clap(possible_values = CapabilityOwner::values())]
        owner: CapabilityOwner,
    },
}

#[derive(Debug, Clone, clap::ArgEnum)]
pub enum CapabilityOwner {
    Client,
    Server,
}

impl CapabilityOwner {
    pub fn values() -> Vec<&'static str> {
        vec!["client", "server"]
    }
}

impl FromStr for CapabilityOwner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "client" => Ok(Self::Client),
            "server" => Ok(Self::Server),
            _ => bail!("Unsupported capability owner!"),
        }
    }
}

impl ShowCommand {
    pub fn handle(&self, cli_api: &mut impl CliManagerCommandApi) -> Result<()> {
        match self {
            ShowCommand::Capabilities { owner } => match owner {
                CapabilityOwner::Client => {
                    println!("Configured {} capabilities:", "client".cyan());
                    for cap in cli_api.config().client_capabilities() {
                        println!("\t{}", cap)
                    }
                }
                CapabilityOwner::Server => match cli_api.pending_session() {
                    Some(session) => match session.server_capabilities() {
                        Some(caps) => {
                            println!("Advertised {} session capabilities:", "server".cyan());
                            for cap in caps {
                                println!("\t{}", cap);
                            }
                        }
                        None => println!("{}", "No server capabilities advertised!".yellow()),
                    },
                    None => {
                        println!("{}", NO_SESSION_ERROR_STR.yellow());
                    }
                },
            },
        };
        Ok(())
    }
}
