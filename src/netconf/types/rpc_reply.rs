use serde::{Deserialize, Serialize};

use super::RpcError;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RpcReply {
    Ok,
    RpcError(RpcError),
}
