pub const XMLNS: &str = "urn:ietf:params:xml:ns:netconf:base:1.0";

use anyhow::{bail, Result};
use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};
use std::io::Cursor;

/// Find the outermost tag with specified name,
/// and return the string slice covering it and all of its contents.
pub(crate) fn get_tag_slice<'i>(input: &'i str, tag: &str) -> Result<&'i str> {
    let start_index = if let Some(index) = input.find(&format!("<{}", tag)) {
        index
    } else {
        bail!("Missing opening tag");
    };

    let closing_str = format!("</{}>", tag);
    let stop_index = if let Some(index) = input.find(&closing_str) {
        index + closing_str.len()
    } else {
        bail!("Missing closing tag");
    };

    if start_index > stop_index {
        bail!("Mismatched order of opening/closing tag")
    }
    Ok(&input[start_index..stop_index])
}

pub(crate) enum RpcWrapMode<'a> {
    Wrapped(&'a str, &'a str),
    Plain,
}

pub(crate) fn xml_events_to_string(events: &[Event], wrap: RpcWrapMode) -> Result<String> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    if let RpcWrapMode::Wrapped(message_id, xmlns) = wrap {
        let mut head_elem = BytesStart::borrowed(b"rpc", b"rpc".len());
        head_elem.push_attribute(("message-id", message_id));
        head_elem.push_attribute(("xmlns", xmlns));
        writer.write_event(Event::Start(head_elem))?;
    }

    for event in events {
        writer.write_event(event)?;
    }

    if let RpcWrapMode::Wrapped(_, _) = wrap {
        writer.write_event(Event::End(BytesEnd::borrowed(b"rpc")))?;
    }

    let rpc_bytes = writer.into_inner().into_inner();
    Ok(String::from_utf8(rpc_bytes)?)
}
