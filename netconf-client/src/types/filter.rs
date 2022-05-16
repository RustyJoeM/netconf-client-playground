use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use crate::{
    common::{xml_events_to_string, RpcWrapMode},
    messages::ToRawXml,
};

/// Composite wrapper for actual [`Filter`] contents & optional namespaces definitions.
#[derive(Debug, Clone)]
pub struct FilterPayload {
    /// Filter contents.
    pub filter: Filter,
    /// List of namespaces used in the filter contents.
    /// Use empty vector if no namespaces are needed/used.
    ///
    /// Each namespace is defined as a tuple of (prefix, value).
    pub namespaces: Vec<(String, String)>,
}

/// Filter used for NETCONF <get>, <get-config> operation
#[derive(Debug, Clone)]
pub enum Filter {
    /// Raw XML dump format of subtree filter, as defined in [RFC 6241 - section 6.2](https://datatracker.ietf.org/doc/html/rfc6241#section-6.2)
    Subtree(String),
    /// Usable only when client supports the [`Capability::XPath`] capability.
    /// (see [RFC 6241](https://datatracker.ietf.org/doc/html/rfc6241#section-8.9), section 8.9)
    Xpath(String),
}

impl ToRawXml for FilterPayload {
    /// Is NOT a NetconfRequest trait impl!
    /// As this field is manually serialized due to cumbersome structure
    /// that current quick_xml cannot handle easily with derives...
    fn to_raw_xml(&self) -> anyhow::Result<String> {
        let mut events: Vec<Event> = vec![];

        let filter_tag = b"filter";

        let mut elem = BytesStart::borrowed(filter_tag, filter_tag.len());
        for (prefix, namespace) in self.namespaces.iter() {
            elem.push_attribute((prefix.as_str(), namespace.as_str()));
        }

        match &self.filter {
            Filter::Subtree(subtree) => {
                elem.push_attribute(("type", "subtree"));
                events.push(Event::Start(elem));
                let data_tag = b"data";
                events.push(Event::Start(BytesStart::borrowed(data_tag, data_tag.len())));
                events.push(Event::Text(BytesText::from_escaped_str(&*subtree)));
                events.push(Event::End(BytesEnd::borrowed(data_tag)));
                events.push(Event::End(BytesEnd::borrowed(filter_tag)));
            }
            Filter::Xpath(xpath) => {
                elem.push_attribute(("type", "xpath"));
                elem.push_attribute(("select", xpath.as_str()));
                events.push(Event::Empty(elem));
            }
        };

        xml_events_to_string(&events, RpcWrapMode::Plain)
    }
}
