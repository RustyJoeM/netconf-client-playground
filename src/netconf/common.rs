pub const XMLNS: &str = "urn:ietf:params:xml:ns:netconf:base:1.0";
pub const MESSAGE_SEPARATOR: &str = "]]>]]>";

use anyhow::{bail, Result};

/// Find the outermost tag with specified name,
/// and return the string slice covering it and all of its contents.
pub fn get_tag_slice<'i>(input: &'i str, tag: &str) -> Result<&'i str> {
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
