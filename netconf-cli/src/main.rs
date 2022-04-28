// #![deny(missing_docs)]
// #![warn(missing_docs)]
// #![deny(missing_doc_code_examples)]

mod clap_args;
use clap_args::Command;

use anyhow::Result;

use clap::StructOpt;
use cli_state::CliState;
use netconf_client::{
    types::{Capability, RpcReply},
    NetconfSession, SshAuthentication,
};
use rustyline::error::ReadlineError;

mod cli_state;

fn handle_command(state: &mut CliState, command: clap_args::Command) -> Result<()> {
    match command {
        Command::HistoryClear {} => {
            state.editor.clear_history();
        }
        Command::Hello {
            address,
            port,
            user,
            password,
        } => {
            let session = NetconfSession::initialize(
                address,
                port,
                // TODO - get auth from opts
                SshAuthentication::UserPassword("admin".to_string(), "admin".to_string()),
                // TODO - client capabilities;
                vec![Capability::Base],
            )?;
            state.set_session(Some(session));
        }
        Command::Lock { target } => {
            if let Some(session) = state.mut_session() {
                let response = session.request_lock(target)?;
                println!("{:?}", response);
            }
        }
        Command::Unlock { target } => {
            if let Some(session) = &mut state.mut_session() {
                let response = session.request_unlock(target)?;
                println!("{:?}", response);
            }
        }
        Command::CloseSession {} => {
            if let Some(session) = &mut state.mut_session() {
                let response = session.request_close_session()?;
                println!("{:?}", response);
                if response.reply == RpcReply::Ok {
                    state.set_session(None);
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut cli_state = CliState::new();

    loop {
        match cli_state.read_line() {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                cli_state.editor.add_history_entry(line.as_str());
                // println!("{}{}", prompt_str, line);
                match clap_args::Args::try_parse_from(line.split_whitespace()) {
                    Ok(matches) => {
                        handle_command(&mut cli_state, matches.command)?;
                    }
                    Err(err) => {
                        // println!("{:?}", err.kind());
                        println!("{}", err.to_string().trim());

                        // let kind = err.kind().to_string();
                        // if !kind.is_empty() {
                        //     println!("{}", kind);
                        // }
                        // let ctx: Vec<_> = err.context().collect();
                        // if ctx.is_empty() {
                        //     println!("{}", err.to_string().trim());
                        // } else {
                        //     for c in ctx {
                        //         // println!("----");
                        //         // println!("{:?}", c.0);
                        //         println!("{:?}", c.1);
                        //     }
                        // }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
