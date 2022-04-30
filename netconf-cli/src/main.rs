// #![deny(missing_docs)]
// #![warn(missing_docs)]
// #![deny(missing_doc_code_examples)]

mod clap_args;

use anyhow::Result;
use clap::StructOpt;
use cli_state::CliState;
use rustyline::error::ReadlineError;

mod cli_state;

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
                        cli_state.handle_command(matches.command)?;
                    }
                    Err(err) => {
                        // println!("{:?}", err.kind());
                        // dbg!(&err);
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
