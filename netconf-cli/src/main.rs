// #![deny(missing_docs)]
// #![warn(missing_docs)]
// #![deny(missing_doc_code_examples)]

mod commands;
mod reedline_helpers;

use anyhow::Result;
use clap::StructOpt;

use colored::Colorize;
use commands::CommandHandler;

fn main() -> Result<()> {
    let mut command_handler = CommandHandler::new();

    loop {
        match command_handler.read_line() {
            Ok(signal) => {
                match signal {
                    reedline::Signal::Success(line) => {
                        if line.is_empty() {
                            continue;
                        }
                        // command_handler.editor.add_history_entry(line.as_str());
                        // println!("{}{}", prompt_str, line);
                        match commands::RootArgs::try_parse_from(line.split_whitespace()) {
                            Ok(matches) => match command_handler.handle_command(&matches.command) {
                                Ok(_) => {}
                                Err(err) => println!("{}", err.to_string().trim().yellow()),
                            },
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
                    reedline::Signal::CtrlC => {
                        println!("CTRL-C");
                        break;
                    }
                    reedline::Signal::CtrlD => {
                        println!("CTRL-D");
                        break;
                    }
                    reedline::Signal::CtrlL => todo!(),
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
