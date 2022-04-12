use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Wrapper structure to allow serialization of parent struct field `field` of type T into `<field>T.to_string()</field>
// TODO - check whether description is correct?
pub struct TagWrapper<T> {
    #[serde(rename = "$value")]
    pub item: T,
}

impl<T> From<T> for TagWrapper<T> {
    fn from(item: T) -> Self {
        Self { item }
    }
}

impl<T> TagWrapper<T> {
    pub fn new(item: T) -> Self {
        Self { item }
    }
}
