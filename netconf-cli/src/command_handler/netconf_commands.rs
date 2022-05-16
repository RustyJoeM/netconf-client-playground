use anyhow::{bail, Result};
use clap::{ArgGroup, Subcommand};
use colored::{Color, Colorize};
use netconf_client::{
    messages::{
        cancel_commit::CancelCommitRequest,
        close_session::CloseSessionRequest,
        copy_config::CopyConfigRequest,
        delete_config::DeleteConfigRequest,
        discard_changes::DiscardChangesRequest,
        get::GetRequest,
        get_config::GetConfigRequest,
        hello::HelloRequest,
        kill_session::KillSessionRequest,
        lock::LockRequest,
        raw_to_pretty_xml,
        unlock::UnlockRequest,
        validate::{ValidateRequest, ValidateSource},
        FullResponse, NetconfResponse, ToRawXml,
    },
    types::{Capability, ConfigWaypoint, Datastore, Filter, FilterPayload},
    NetconfSession, SshAuthentication,
};
use std::net::IpAddr;

use crate::cli_manager::{CliManagerCommandApi, DumpXmlFormat};

pub const NO_SESSION_ERROR_STR: &str = "There is no opened NETCONF session!";

#[derive(Subcommand, Debug)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub enum NetconfCommand {
    /// Initiate connection to target server with <hello> message exchange.
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
    /// The <lock> operation allows the client to lock the entire configuration datastore system of a device.
    Lock {
        #[clap(possible_values = Datastore::values())]
        target: Datastore,
    },
    /// The <unlock> operation is used to release a configuration lock, previously obtained with the <lock> operation.
    Unlock {
        #[clap(possible_values = Datastore::values())]
        target: Datastore,
    },
    /// Retrieve running configuration and device state information.
    Get {
        #[clap(subcommand)]
        filter: Option<FilterCommand>,
    },
    /// Retrieve all or part of a specified configuration datastore.
    GetConfig {
        /// Datastore containing the requested configuration.
        #[clap(possible_values = Datastore::values())]
        source: Datastore,
        /// Optional filter to retrieve specified part of config.
        #[clap(subcommand)]
        filter: Option<FilterCommand>,
    },
    EditConfig {},
    /// Create or replace an entire configuration datastore with the contents of another complete configuration datastore.
    #[clap(group(
        ArgGroup::new("from")
            .required(true)
            .multiple(false)
            .args(&["from-datastore", "from-url"]),
    ), group(
        ArgGroup::new("to")
            .required(true)
            .multiple(false)
            .args(&["to-datastore", "to-url"]),
    ))]
    CopyConfig {
        /// Source datastore to be copied from.
        #[clap(long, possible_values = Datastore::values())]
        from_datastore: Option<Datastore>,
        /// Source alternative - URL of configuration to be copied from, for `:url` capability enabled servers.
        #[clap(long)]
        from_url: Option<String>,
        /// Target datastore to be written to.
        #[clap(long, possible_values = Datastore::values())]
        to_datastore: Option<Datastore>,
        /// Target alternative - URL of configuration to be copied into, for `:url` capability enabled servers.
        #[clap(long)]
        to_url: Option<String>,
    },
    #[clap(group(
        ArgGroup::new("from")
            .required(true)
            .multiple(false)
            .args(&["datastore", "url"]),
    ))]
    /// Delete a configuration datastore. The <running> configuration datastore cannot be deleted.
    DeleteConfig {
        /// Target datastore.
        #[clap(long, possible_values = Datastore::values())]
        datastore: Option<Datastore>,
        /// URL of the target, for  :url capability enabled servers.
        #[clap(long)]
        url: Option<String>,
    },
    /// Terminate remote session via <kill-session> operation & session-id.
    KillSession {
        session_id: u32,
    },
    Commit {},
    /// The <discard-changes> operation can be used to revert the candidate configuration to the current running configuration.
    DiscardChanges {},
    /// Cancels an ongoing confirmed commit.  If the <persist-id>
    /// parameter is not given, the <cancel-commit> operation MUST be
    /// issued on the same session that issued the confirmed commit.
    CancelCommit {
        persist_id: Option<u32>,
    },
    /// Validates the contents of the specified configuration.
    Validate {
        #[clap(subcommand)]
        source: ValidateSourceCommand,
    },
    /// Dispatch <close-session> request for currently opened NETCONF session.
    CloseSession {},
}

impl NetconfCommand {
    pub fn to_raw_xml(
        &self,
        cli_api: &impl CliManagerCommandApi,
        message_id: String,
    ) -> Result<String> {
        match self {
            NetconfCommand::Hello {
                address: _,
                port: _,
                user: _,
                password: _,
            } => {
                let capabilities = cli_api.config().client_capabilities().to_owned();
                let request = HelloRequest::new(capabilities);
                request.to_raw_xml()
            }
            NetconfCommand::Lock { target } => {
                LockRequest::new(message_id, target.clone()).to_raw_xml()
            }
            NetconfCommand::Unlock { target } => {
                UnlockRequest::new(message_id, target.clone()).to_raw_xml()
            }
            NetconfCommand::Get { filter } => {
                let filter = filter.as_ref().map(|f| f.into());
                GetRequest::new(message_id, filter).to_raw_xml()
            }
            NetconfCommand::GetConfig { source, filter } => {
                let filter = filter.as_ref().map(|f| f.into());
                GetConfigRequest::new(message_id, source.clone(), filter).to_raw_xml()
            }
            NetconfCommand::EditConfig {} => todo!(),
            NetconfCommand::CopyConfig {
                from_datastore,
                from_url,
                to_datastore,
                to_url,
            } => {
                let source = args_to_config_waypoint(from_datastore, from_url)?;
                let target = args_to_config_waypoint(to_datastore, to_url)?;
                CopyConfigRequest::new(message_id, target, source).to_raw_xml()
            }
            NetconfCommand::DeleteConfig { datastore, url } => {
                let target = args_to_config_waypoint(datastore, url)?;
                DeleteConfigRequest::new(message_id, target).to_raw_xml()
            }
            NetconfCommand::KillSession { session_id } => {
                KillSessionRequest::new(message_id, *session_id).to_raw_xml()
            }
            NetconfCommand::Commit {} => todo!(),
            NetconfCommand::DiscardChanges {} => {
                DiscardChangesRequest::new(message_id).to_raw_xml()
            }
            NetconfCommand::CancelCommit { persist_id } => {
                CancelCommitRequest::new(message_id, *persist_id).to_raw_xml()
            }
            NetconfCommand::Validate { source } => {
                let source = source.to_validate_source();
                ValidateRequest::new(message_id, source).to_raw_xml()
            }
            NetconfCommand::CloseSession {} => CloseSessionRequest::new(message_id).to_raw_xml(),
        }
    }

    pub fn dump_command_xml(
        &self,
        cli_api: &mut impl CliManagerCommandApi,
        dump_mode: DumpXmlFormat,
        header: &str,
        message_id: &str,
    ) -> Result<()> {
        let dump_string = match dump_mode {
            DumpXmlFormat::None => None,
            DumpXmlFormat::Raw => Some(self.to_raw_xml(cli_api, message_id.to_string())?),
            DumpXmlFormat::Pretty => {
                let raw = self.to_raw_xml(cli_api, message_id.to_string())?;
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
            if cli_api.pending_session().is_some() {
                bail!("There is a pending session opened already! Close it via <close-session> request first to initiate a new one...");
            }

            let auth = SshAuthentication::UserPassword(user.to_owned(), password.to_owned());

            let client_capabilities: Vec<Capability> =
                cli_api.config().client_capabilities().to_vec();

            let mut session =
                NetconfSession::new(*address, *port, auth, client_capabilities.clone());
            session.connect()?;
            println!("SSH connected to target NETCONF server.");

            let request = HelloRequest::new(client_capabilities);
            let _ = self.dump_command_xml(cli_api, request_dump_mode, "Request:", "0");

            // do NOT validate hello request -> no server capabilities yet to validate against!
            let do_validate_backup = session.validate_capabilities();
            session.set_validate_capabilities(false);
            let response = session.dispatch_request(request)?;
            session.set_validate_capabilities(do_validate_backup);
            let _ = dump_response(response_dump_mode, &response);

            if response.typed.succeeded() {
                session.update_on_hello(&response.typed)?;
                println!(
                    "Hello capability exchange successful, base capability: {}",
                    format!("{}", session.base_capability()).cyan()
                );
                cli_api.set_pending_session(Some(session));
                return Ok(());
            }

            bail!("Failed to connect to NETCONF server!")
        };

        let message_id = match cli_api.pending_session_mut() {
            Some(session) => session.new_message_id(),
            None => bail!(NO_SESSION_ERROR_STR),
        };

        let _ = self.dump_command_xml(cli_api, request_dump_mode, "Request:", &message_id);

        let pending_session = cli_api.pending_session_mut().unwrap();

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
            NetconfCommand::CopyConfig {
                from_datastore,
                from_url,
                to_datastore,
                to_url,
            } => {
                let source = args_to_config_waypoint(from_datastore, from_url)?;
                let target = args_to_config_waypoint(to_datastore, to_url)?;
                let request = CopyConfigRequest::new(message_id, target, source);
                let response = pending_session.dispatch_request(request)?;
                let _ = dump_response(response_dump_mode, &response);
            }
            NetconfCommand::DeleteConfig { datastore, url } => {
                let target = args_to_config_waypoint(datastore, url)?;
                let request = DeleteConfigRequest::new(message_id, target);
                let response = pending_session.dispatch_request(request)?;
                let _ = dump_response(response_dump_mode, &response);
            }
            NetconfCommand::DiscardChanges {} => {
                let request = DiscardChangesRequest::new(message_id);
                let response = pending_session.dispatch_request(request)?;
                let _ = dump_response(response_dump_mode, &response);
            }
            NetconfCommand::CancelCommit { persist_id } => {
                let request = CancelCommitRequest::new(message_id, *persist_id);
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
            NetconfCommand::Get { filter } => {
                let filter = filter.as_ref().map(|f| f.into());
                let request = GetRequest::new(message_id, filter);
                let response = pending_session.dispatch_request(request)?;
                let _ = dump_response(response_dump_mode, &response);
            }
            NetconfCommand::GetConfig { source, filter } => {
                let filter = filter.as_ref().map(|f| f.into());
                let request = GetConfigRequest::new(message_id, source.to_owned(), filter);
                let response = pending_session.dispatch_request(request)?;
                let _ = dump_response(response_dump_mode, &response);
            }
            NetconfCommand::Validate { source } => {
                let source = source.to_validate_source();
                let request = ValidateRequest::new(message_id, source);
                let response = pending_session.dispatch_request(request)?;
                let _ = dump_response(response_dump_mode, &response);
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
        match response.typed.succeeded() {
            true => println!("{}", &s),
            false => println!("{}", &s.yellow()),
        };
    }
    Ok(())
}

/// Build `netconf-client`'s `ConfigWaypoint` struct from pair of exclusive CLAP arguments.
fn args_to_config_waypoint(
    datastore: &Option<Datastore>,
    url: &Option<String>,
) -> Result<ConfigWaypoint> {
    if let Some(datastore) = datastore {
        Ok(ConfigWaypoint::Datastore(datastore.clone()))
    } else if let Some(url) = url {
        Ok(ConfigWaypoint::Url(url.to_string()))
    } else {
        bail!("Failed to parse source!");
    }
}

#[derive(clap::Parser, Debug)]
pub enum FilterCommand {
    /// Specify items with [subtree](https://datatracker.ietf.org/doc/html/rfc6241#section-6) filtering.
    Subtree { value: String },
    /// Filter using [xpath(https://datatracker.ietf.org/doc/html/rfc6241#section-8.9) expression.
    #[clap(name = "xpath")]
    XPath { value: String },
}

impl From<&FilterCommand> for FilterPayload {
    fn from(cmd: &FilterCommand) -> Self {
        let value = match cmd {
            FilterCommand::Subtree { value } => Filter::Subtree(value.to_string()),
            FilterCommand::XPath { value } => Filter::Xpath(value.to_string()),
        };
        FilterPayload {
            filter: value,
            namespaces: vec![],
        }
    }
}

#[derive(clap::Parser, Debug)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub enum ValidateSourceCommand {
    /// Datastore to be validate.
    Datastore {
        #[clap(possible_values = Datastore::values())]
        value: Datastore,
    },
    /// Complete configuration to be validated.
    Config { value: String },
    /// Url of the configuration for validation.
    Url { value: String },
}

impl ValidateSourceCommand {
    pub fn to_validate_source(&self) -> ValidateSource {
        match self {
            ValidateSourceCommand::Datastore { value } => {
                ValidateSource::Datastore(value.to_owned())
            }
            ValidateSourceCommand::Config { value } => ValidateSource::Config(value.to_owned()),
            ValidateSourceCommand::Url { value } => ValidateSource::Url(value.to_owned()),
        }
    }
}
