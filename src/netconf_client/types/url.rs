use serde::Serialize;

use super::tag_wrapper::TagWrapper;

#[derive(Debug, Serialize)]
pub struct UrlParam {
    pub inner: String,
}

impl From<TagWrapper<String>> for UrlParam {
    fn from(wrapper: TagWrapper<String>) -> Self {
        Self {
            inner: wrapper.item,
        }
    }
}

// TODO - implement parser for supported schemas... (see RFC)
// impl UrlParam {}
