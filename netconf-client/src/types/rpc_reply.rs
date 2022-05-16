use super::RpcError;

/// Result-like type typically parsed from NETCONF response RPC.
#[derive(Debug, Clone, PartialEq)]
pub enum RpcReply {
    /// Variant corresponding to <ok/> RPC.
    Ok,
    /// Variant containing the <rpc-error> data.
    Error(RpcError),
}
