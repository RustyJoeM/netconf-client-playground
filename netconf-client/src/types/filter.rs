use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use crate::{
    common::{xml_events_to_string, RpcWrapMode},
    messages::NetconfRequest,
};

#[derive(Debug, Clone)]
pub struct Filter {
    pub value: FilterType,
    pub namespaces: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub enum FilterType {
    Subtree(String),
    /// Usable only when client supports the [`Capability::XPath`] capability.
    /// (see [RFC 6241](https://datatracker.ietf.org/doc/html/rfc6241#section-8.9), section 8.9)
    Xpath(String),
}

impl NetconfRequest for Filter {
    fn to_netconf_rpc(&self) -> anyhow::Result<String> {
        let mut events: Vec<Event> = vec![];

        let filter_tag = b"filter";

        let mut elem = BytesStart::borrowed(filter_tag, filter_tag.len());
        for (prefix, namespace) in self.namespaces.iter() {
            elem.push_attribute((prefix.as_str(), namespace.as_str()));
        }

        match &self.value {
            FilterType::Subtree(subtree) => {
                elem.push_attribute(("type", "subtree"));
                events.push(Event::Start(elem));
                let data_tag = b"data";
                events.push(Event::Start(BytesStart::borrowed(data_tag, data_tag.len())));
                events.push(Event::Text(BytesText::from_escaped_str(&*subtree)));
                events.push(Event::End(BytesEnd::borrowed(data_tag)));
                events.push(Event::End(BytesEnd::borrowed(filter_tag)));
            }
            FilterType::Xpath(xpath) => {
                elem.push_attribute(("type", "xpath"));
                elem.push_attribute(("select", xpath.as_str()));
                events.push(Event::Empty(elem));
            }
        };

        xml_events_to_string(&events, RpcWrapMode::Plain)
    }
}
