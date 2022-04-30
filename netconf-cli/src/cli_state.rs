use crate::clap_args::Command;
use colored::{Color, Colorize};
use netconf_client::{
    messages::{
        close_session::CloseSessionRequest, lock::LockRequest, unlock::UnlockRequest, FullResponse,
        NetconfRequest, NetconfResponse,
    },
    types::{Capability, RpcReply},
    NetconfSession, SshAuthentication,
};
use rustyline::{error::ReadlineError, Editor};
// use std::io::Write;
// use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub const HISTORY_FILE: &str = "netconf-cli-history.txt";

pub struct CliState {
    // TODO - make private
    pub editor: Editor<()>,
    session: Option<NetconfSession>,
    prompt: String,
}

impl CliState {
    pub fn new() -> Self {
        let mut editor = Editor::<()>::new();
        if editor.load_history(HISTORY_FILE).is_err() {
            println!("No previous history.");
        }
        let mut instance = Self {
            editor,
            session: None,
            prompt: "".to_string(),
        };
        instance.update_prompt();
        instance
    }

    pub fn set_session(&mut self, session: Option<NetconfSession>) {
        self.session = session;
        self.update_prompt();
    }

    fn update_prompt(&mut self) {
        self.prompt = match &self.session {
            Some(session) => {
                let session_id = session.session_id().unwrap_or(0);
                format!(
                    "--> NETCONF(session-id:{}@{})$ ",
                    session_id,
                    session.target_string()
                )
            }
            None => "--> NETCONF$ ".to_string(),
        };
    }

    pub fn read_line(&mut self) -> Result<String, ReadlineError> {
        let r = self.editor.readline(&self.prompt.red())?;
        Ok(r)
    }

    pub fn log_request(request: &impl NetconfRequest) {
        // TODO - if self not logging
        match request.prettified_rpc() {
            Ok(s) => {
                // print_colored("Request:", Color::Green);
                println!("{}", "Request:".green());
                println!("{}", &s);
            }
            Err(_e) => todo!(),
        }
    }

    pub fn log_response<R: NetconfResponse + std::fmt::Debug>(response: &FullResponse<R>) {
        match response.prettified_rpc() {
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

    pub fn handle_command(&mut self, command: Command) -> anyhow::Result<()> {
        match command {
            Command::HistoryClear {} => {
                self.editor.clear_history();
            }
            Command::Hello {
                address,
                port,
                user,
                password,
            } => {
                let auth = SshAuthentication::UserPassword(user, password);
                // TODO - client capabilities;
                let client_capabilities = vec![Capability::Base];
                // TODO - dump requests/responses of nested initialize block!
                let session = NetconfSession::initialize(address, port, auth, client_capabilities)?;
                self.set_session(Some(session));
            }
            Command::Lock { target } => {
                if let Some(session) = &mut self.session {
                    let message_id = session.new_message_id();
                    let request = LockRequest::new(message_id, target);
                    Self::log_request(&request);

                    let response = session.dispatch_request(request)?;
                    Self::log_response(&response);
                }
            }
            Command::Unlock { target } => {
                if let Some(session) = &mut self.session {
                    let message_id = session.new_message_id();
                    let request = UnlockRequest::new(message_id, target);
                    Self::log_request(&request);

                    let response = session.dispatch_request(request)?;
                    Self::log_response(&response);
                }
            }
            Command::CloseSession {} => {
                if let Some(session) = &mut self.session {
                    let message_id = session.new_message_id();
                    let request = CloseSessionRequest::new(message_id);
                    Self::log_request(&request);

                    let response = session.dispatch_request(request)?;
                    Self::log_response(&response);
                    if response.typed.reply == RpcReply::Ok {
                        self.set_session(None);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Drop for CliState {
    fn drop(&mut self) {
        if let Some(session) = &mut self.session {
            session.request_close_session();
        }
        self.editor.save_history(HISTORY_FILE).unwrap();
    }
}

// fn print_colored(s: &str, color: Color) -> anyhow::Result<()> {
//     let mut stdout = StandardStream::stdout(ColorChoice::Always);
//     stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
//     writeln!(&mut stdout, "{}", s);
//     Ok(())
// }
