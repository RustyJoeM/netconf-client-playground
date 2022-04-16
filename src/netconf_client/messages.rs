mod close_session;
pub use close_session::{CloseSessionRequest, CloseSessionResponse};

mod copy_config;
pub use copy_config::{CopyConfigRequest, CopyConfigResponse};

mod delete_config;
pub use delete_config::{DeleteConfigRequest, DeleteConfigResponse};

mod edit_config;
pub use edit_config::{EditConfigParams, EditConfigRequest, EditConfigResponse};

mod get;
pub use get::{GetRequest, GetResponse};

mod get_config;
pub use get_config::{GetConfigRequest, GetConfigResponse};

mod hello;
pub use hello::{HelloRequest, HelloResponse};

mod kill_session;
pub use kill_session::{KillSessionRequest, KillSessionResponse};

mod lock;
pub use lock::{LockRequest, LockResponse};

mod unlock;
pub use unlock::{UnlockRequest, UnlockResponse};

mod commit;
pub use commit::{CommitRequest, CommitResponse, ConfirmedCommitParams};

mod discard_changes;
pub use discard_changes::{DiscardChangesRequest, DiscardChangesResponse};

mod cancel_commit;
pub use cancel_commit::{CancelCommitRequest, CancelCommitResponse};

use anyhow::Result;

pub trait NetconfRequest {
    fn to_netconf_rpc(&self) -> Result<String>;
}

pub trait NetconfResponse {
    fn from_netconf_rpc(s: String) -> Self;
}
