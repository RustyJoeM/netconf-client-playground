pub mod cancel_commit;
pub mod close_session;
pub mod commit;
pub mod copy_config;
pub mod delete_config;
pub mod discard_changes;
pub mod edit_config;
pub mod get;
pub mod get_config;
pub mod hello;
pub mod kill_session;
pub mod lock;
pub mod unlock;
pub mod validate;

use anyhow::Result;

pub trait NetconfRequest {
    fn to_netconf_rpc(&self) -> Result<String>;
}

pub trait NetconfResponse {
    fn from_netconf_rpc(s: String) -> Result<Self>
    where
        Self: Sized;
}
