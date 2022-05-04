use std::net::IpAddr;

use clap::{Parser, Subcommand};
use netconf_client::{
    messages::{
        close_session::CloseSessionRequest, hello::HelloRequest, kill_session::KillSessionRequest,
        lock::LockRequest, unlock::UnlockRequest, ToRawXml,
    },
    types::{Capability, Datastore},
};

// const HELP_TEMPLATE: &str = "{about}\n{usage}\n{all-args}";

#[derive(Parser, Debug)]
#[clap(
    no_binary_name(true),
    // disable_help_subcommand(true),
    // disable_help_flag(true),
    // help_template("{all-args}")
)]
pub struct RootArgs {
    #[clap(subcommand)]
    pub command: RootCommand,
}

#[derive(Subcommand, Debug)]
pub enum RootCommand {
    // #[clap(
    //     about("Clear the whole previously saved history buffer."),
    //     disable_help_flag(true),
    //     arg_required_else_help = false
    // )]
    // HistoryClear {},
    #[clap(flatten)]
    NetconfRaw(NetconfCommand),
    #[clap(subcommand)]
    Dump(NetconfDump),
}

#[derive(Subcommand, Debug)]
pub enum NetconfDump {
    #[clap(flatten)]
    NetconfRaw(NetconfCommand),
}

#[derive(Subcommand, Debug)]
pub enum NetconfCommand {
    #[clap(
        no_binary_name(true),
        about("<hello> request to initiate connection with NETCONF server"),
        // disable_help_flag(true),
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
    pub fn to_raw_xml(&self, message_id: String) -> anyhow::Result<String> {
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
}

// pub fn to_request(
//     &self,
//     message_id: String,
// ) -> Box<dyn NetconfRequest<Response = Box<dyn NetconfResponseAndMore>>> {
//     match self {
//     }
// }

// pub trait NetconfResponseAndMore: NetconfResponse + std::fmt::Debug {}

// impl NetconfResponse for Box<dyn NetconfResponseAndMore> {
//     fn from_netconf_rpc(s: &str) -> anyhow::Result<Self>
//     where
//         Self: Sized,
//     {
//         Self::from_netconf_rpc(s)
//     }
// }

// impl<T: NetconfResponse + std::fmt::Debug> NetconfResponseAndMore for T {}

// pub struct BoxResponder<T>(T);
// impl<T: NetconfRequest> NetconfRequest for BoxResponder<T> {
//     type Response = Box<dyn NetconfResponseAndMore>;

//     fn to_raw_xml(&self) -> anyhow::Result<String> {
//         Self::to_raw_xml(&self)
//     }
// }
