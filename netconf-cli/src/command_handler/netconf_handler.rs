use anyhow::{bail, Result};
use clap::Subcommand;
use colored::{Color, Colorize};
use netconf_client::{
    messages::{
        close_session::CloseSessionRequest, hello::HelloRequest, kill_session::KillSessionRequest,
        lock::LockRequest, raw_to_pretty_xml, unlock::UnlockRequest, FullResponse, NetconfResponse,
        ToRawXml,
    },
    types::{Capability, Datastore},
    NetconfSession, SshAuthentication,
};
use std::net::IpAddr;

use crate::cli_manager::{CliManagerCommandApi, DumpXmlFormat};

const NO_SESSION_ERROR_STR: &str = "There is no opened NETCONF session!";

#[derive(Subcommand, Debug)]
pub enum NetconfCommand {
    #[clap(
        about("<hello> request to initiate connection with NETCONF server"),
        // disable_help_flag(true),
        // arg_required_else_help = false
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
    /// <lock> operation request for target datastore.
    Lock {
        #[clap(possible_values(["running", "candidate"]))]
        target: Datastore,
    },
    /// <unlock> operation request for target datastore.
    Unlock {
        #[clap(possible_values(["running", "candidate"]))]
        target: Datastore,
    },
    Get {},
    GetConfig {},
    EditConfig {},
    CopyConfig {},
    DeleteConfig {},
    KillSession {
        session_id: u32,
    },
    Commit {},
    DiscardChanges {},
    CancelCommit {
        persist_id: Option<u32>,
    },
    Validate {},
    #[clap(about("Dispatch <close-session> request for currently opened NETCONF session."))]
    CloseSession {},
}

impl NetconfCommand {
    pub fn to_raw_xml(&self, message_id: String) -> Result<String> {
        match self {
            NetconfCommand::Hello {
                address: _,
                port: _,
                user: _,
                password: _,
            } => {
                let request = HelloRequest::new(vec![Capability::Base]);
                request.to_raw_xml()
            }
            NetconfCommand::Lock { target } => {
                LockRequest::new(message_id, target.clone()).to_raw_xml()
            }
            NetconfCommand::Unlock { target } => {
                UnlockRequest::new(message_id, target.clone()).to_raw_xml()
            }
            NetconfCommand::Get {} => todo!(),
            NetconfCommand::GetConfig {} => todo!(),
            NetconfCommand::EditConfig {} => todo!(),
            NetconfCommand::CopyConfig {} => todo!(),
            NetconfCommand::DeleteConfig {} => todo!(),
            NetconfCommand::KillSession { session_id } => {
                KillSessionRequest::new(message_id, *session_id).to_raw_xml()
            }
            NetconfCommand::Commit {} => todo!(),
            NetconfCommand::DiscardChanges {} => todo!(),
            NetconfCommand::CancelCommit { persist_id: _ } => todo!(),
            NetconfCommand::Validate {} => todo!(),
            NetconfCommand::CloseSession {} => CloseSessionRequest::new(message_id).to_raw_xml(),
        }
    }

    pub fn dump_command_xml(
        &self,
        dump_mode: DumpXmlFormat,
        header: &str,
        message_id: &str,
    ) -> Result<()> {
        let dump_string = match dump_mode {
            DumpXmlFormat::None => None,
            DumpXmlFormat::Raw => Some(self.to_raw_xml(message_id.to_string())?),
            DumpXmlFormat::Pretty => {
                let raw = self.to_raw_xml(message_id.to_string())?;
                Some(raw_to_pretty_xml(&raw)?)
            }
        };
        if let Some(s) = dump_string {
            println!("{}", header.green());
            println!("{}", &s);
        }
        Ok(())
    }

    pub fn handle(&self, cli_api: &mut impl CliManagerCommandApi) -> Result<()> {
        let request_dump_mode = cli_api.config().request_dump_mode();
        let response_dump_mode = cli_api.config().response_dump_mode();

        if let NetconfCommand::Hello {
            address,
            port,
            user,
            password,
        } = &self
        {
            let auth = SshAuthentication::UserPassword(user.to_owned(), password.to_owned());
            // TODO - client capabilities;
            let client_capabilities = vec![Capability::Base];
            let mut session =
                NetconfSession::new(*address, *port, auth, client_capabilities.clone());
            session.connect()?;
            println!("Connected to target NETCONF server.");

            let request = HelloRequest::new(client_capabilities);
            let _ = self.dump_command_xml(request_dump_mode, "Request:", "0");

            let response = session.dispatch_request(request)?;
            let _ = dump_response(response_dump_mode, &response);

            cli_api.set_pending_session(Some(session));
            return Ok(());
        };

        let pending_session = match cli_api.pending_session_mut() {
            Some(session) => session,
            None => bail!(NO_SESSION_ERROR_STR),
        };

        let message_id = pending_session.new_message_id();

        let _ = self.dump_command_xml(request_dump_mode, "Request:", &message_id);
        match &self {
            NetconfCommand::Lock { target } => {
                let request = LockRequest::new(message_id, target.clone());
                let response = pending_session.dispatch_request(request)?;
                let _ = dump_response(response_dump_mode, &response);
            }
            NetconfCommand::Unlock { target } => {
                let request = UnlockRequest::new(message_id, target.clone());
                let response = pending_session.dispatch_request(request)?;
                let _ = dump_response(response_dump_mode, &response);
            }
            NetconfCommand::CloseSession {} => {
                let request = CloseSessionRequest::new(message_id);
                let response = pending_session.dispatch_request(request)?;
                let _ = dump_response(response_dump_mode, &response);
                if response.typed.succeeded() {
                    cli_api.set_pending_session(None);
                }
            }
            _ => {}
        }

        Ok(())
    }
}

pub fn dump_response<R: NetconfResponse>(
    dump_mode: DumpXmlFormat,
    response: &FullResponse<R>,
) -> Result<()> {
    // TODO - add Cow for lighter handling?
    let dump_string = match dump_mode {
        DumpXmlFormat::None => None,
        DumpXmlFormat::Raw => Some(response.dump.clone()),
        DumpXmlFormat::Pretty => Some(raw_to_pretty_xml(&response.dump)?),
    };
    if let Some(s) = dump_string {
        let header_color = match response.typed.succeeded() {
            true => Color::Green,
            false => Color::BrightRed,
        };
        println!("{}", "Response:".color(header_color));
        println!("{}", &s);
    }
    Ok(())
}
