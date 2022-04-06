use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct GetRequest {
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: String,
}
