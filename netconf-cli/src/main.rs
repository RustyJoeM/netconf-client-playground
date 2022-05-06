// #![deny(missing_docs)]
// #![warn(missing_docs)]
// #![deny(missing_doc_code_examples)]

mod cli_manager;
mod command_handler;

use anyhow::Result;
use clap::StructOpt;
use cli_manager::CliManager;
use command_handler::{CommandHandler, RootArgs};

use colored::Colorize;

fn main() -> Result<()> {
    let mut cli_manager = CliManager::new()?;
    let mut command_handler = CommandHandler::new();

    loop {
        match cli_manager.get_user_input()? {
            reedline::Signal::Success(line) => {
                if line.is_empty() {
                    continue;
                }
                let tokens = match shlex::split(&line) {
                    Some(split) => split,
                    None => {
                        println!(
                            "{}",
                            "Failed to parse command - probably mismatched quotes?".yellow()
                        );
                        continue;
                    }
                };
                // move this into command handler?
                match RootArgs::try_parse_from(tokens) {
                    Ok(matches) => {
                        match command_handler.handle_command(&mut cli_manager, &matches.command) {
                            Ok(_) => {}
                            Err(err) => println!("{}", err.to_string().trim().yellow()),
                        }
                    }
                    Err(err) => {
                        println!("{}", err.to_string().trim());
                    }
                }
            }
            reedline::Signal::CtrlC => {
                println!("CTRL-C");
                break;
            }
            reedline::Signal::CtrlD => {
                println!("CTRL-D");
                break;
            }
        }
    }

    Ok(())
}
