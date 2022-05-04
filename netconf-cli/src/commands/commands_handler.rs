use super::clap_commands::{NetconfCommand, NetconfDump, RootCommand};
use crate::reedline_helpers::custom_prompt::CustomPrompt;
use anyhow::Result;
use colored::{Color, Colorize};
use netconf_client::messages::{raw_to_pretty_xml, ToPrettyXml};
use netconf_client::{
    messages::{
        close_session::CloseSessionRequest, hello::HelloRequest, lock::LockRequest,
        unlock::UnlockRequest, FullResponse, NetconfResponse,
    },
    types::{Capability, RpcReply},
    NetconfSession, SshAuthentication,
};

use reedline::{FileBackedHistory, Reedline, Signal};

pub const HISTORY_FILE: &str = "netconf-cli-history.txt";

pub struct CommandHandler {
    // TODO - make private
    pub editor: Reedline,
    session: Option<NetconfSession>,
    prompt: CustomPrompt,
}

const NO_SESSION_ERROR_STR: &str = "There is no opened NETCONF session!";

const BASE_PROMPT: &str = "netconf-cli";

impl CommandHandler {
    pub fn new() -> Self {
        let history = Box::new(
            FileBackedHistory::with_file(5, HISTORY_FILE.into())
                .expect("Error configuring history with file"),
        );

        // let commands = vec![
        //     "test".into(),
        //     "hello world".into(),
        //     "hello world reedline".into(),
        //     "this is the reedline crate".into(),
        // ];
        // let completer = Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
        // Use the interactive menu to select options from the completer
        // let completion_menu = Box::new(CompletionMenu::default());

        // let completer = CustomCompleter::new();
        // dbg!(&completer);
        // let completion_menu = Box::new(CompletionMenu::default());

        // let hinter = CustomHinter {};

        let editor = Reedline::create()
            .with_history(history)
            // .with_completer(completer)
            // .with_hinter(Box::new(hinter));
        // .with_menu(completion_menu);
        ;

        let prompt = CustomPrompt::new(BASE_PROMPT.to_string());
        let mut instance = Self {
            editor,
            session: None,
            prompt,
        };
        instance.update_prompt();
        instance
    }

    fn set_session(&mut self, session: Option<NetconfSession>) {
        self.session = session;
        self.update_prompt();
    }

    fn update_prompt(&mut self) {
        let new_prompt = match &self.session {
            Some(session) => {
                let session_id = session.session_id().unwrap_or(0);
                format!(
                    "{}(session-id:{}@{})",
                    BASE_PROMPT,
                    session_id,
                    session.target_string()
                )
            }
            None => BASE_PROMPT.to_string(),
        };
        self.prompt.update_prompt(new_prompt);
    }

    pub fn read_line(&mut self) -> anyhow::Result<Signal> {
        // print_colored("", termcolor::Color::Magenta);
        let r = self.editor.read_line(&self.prompt)?;
        // print_colored("", termcolor::Color::White);
        Ok(r)
    }

    pub fn dump_command_xml(
        &self,
        header: &str,
        command: &NetconfCommand,
        message_id: &str,
    ) -> Result<()> {
        // TODO - if self not logging
        match command.to_raw_xml(message_id.to_string()) {
            Ok(s) => {
                let pretty_xml = raw_to_pretty_xml(&s)?;
                // print_colored("Request:", Color::Green);
                println!("{}", header.green());
                println!("{}", &pretty_xml);
            }
            Err(_e) => todo!(),
        }
        Ok(())
    }

    pub fn log_response<R: NetconfResponse>(response: &FullResponse<R>) {
        match response.to_pretty_xml() {
            Ok(s) => {
                let header_color = match response.typed.succeeded() {
                    true => Color::Green,
                    false => Color::BrightRed,
                };
                println!("{}", "Response:".color(header_color));
                println!("{}", &s);
            }
            Err(_) => todo!(),
        }
    }

    pub fn handle_command(&mut self, command: &RootCommand) -> Result<()> {
        // TODO - message-id monotonous increase shared between dump & actual execution
        let message_id = match &mut self.session {
            Some(session) => session.new_message_id(),
            None => "0".to_string(),
        };
        match command {
            RootCommand::NetconfRaw(command) => self.handle_netconf_command(command, message_id),
            RootCommand::Dump(NetconfDump::NetconfRaw(command)) => {
                let _ = self.dump_command_xml("XML dump only:", command, &message_id);
                Ok(())
            }
        }
    }

    fn handle_netconf_command(
        &mut self,
        command: &NetconfCommand,
        message_id: String,
    ) -> Result<()> {
        let _ = self.dump_command_xml("Request:", command, &message_id);

        if let NetconfCommand::Hello {
            address,
            port,
            user,
            password,
        } = command
        {
            let auth = SshAuthentication::UserPassword(user.to_owned(), password.to_owned());
            // TODO - client capabilities;
            let client_capabilities = vec![Capability::Base];
            let mut session =
                NetconfSession::new(*address, *port, auth, client_capabilities.clone());
            session.connect()?;
            println!("Connected to target NETCONF server.");

            let request = HelloRequest::new(client_capabilities);
            let response = session.dispatch_request(request)?;
            Self::log_response(&response);

            self.set_session(Some(session));
            return Ok(());
        };

        if self.session.is_none() {
            anyhow::bail!(NO_SESSION_ERROR_STR);
        }

        match command {
            NetconfCommand::Lock { target } => {
                if let Some(session) = &mut self.session {
                    let request = LockRequest::new(message_id, target.clone());
                    let response = session.dispatch_request(request)?;
                    Self::log_response(&response);
                }
            }
            NetconfCommand::Unlock { target } => {
                if let Some(session) = &mut self.session {
                    let request = UnlockRequest::new(message_id, target.clone());
                    let response = session.dispatch_request(request)?;
                    Self::log_response(&response);
                }
            }
            NetconfCommand::CloseSession {} => {
                if let Some(session) = &mut self.session {
                    let request = CloseSessionRequest::new(message_id);
                    let response = session.dispatch_request(request)?;
                    Self::log_response(&response);
                    if response.typed.reply == RpcReply::Ok {
                        self.set_session(None);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Drop for CommandHandler {
    fn drop(&mut self) {
        if let Some(session) = &mut self.session {
            let _ = session.request_close_session();
        }
        // self.editor.save_history(HISTORY_FILE).unwrap();
        let _ = self.editor.sync_history();
    }
}

// use std::io::Write;
// use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
// fn print_colored(s: &str, color: termcolor::Color) -> anyhow::Result<()> {
//     let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
//     stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(color)))?;
//     writeln!(&mut stdout, "{}", s);
//     Ok(())
// }
