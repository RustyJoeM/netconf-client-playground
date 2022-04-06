use super::RpcError;

#[derive(Debug)]
pub enum RpcReply {
    Ok,
    Error(RpcError),
}
