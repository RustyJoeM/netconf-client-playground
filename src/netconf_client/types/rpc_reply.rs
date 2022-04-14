use super::RpcError;

#[derive(Debug, PartialEq)]
pub enum RpcReply {
    Ok,
    Error(RpcError),
}
