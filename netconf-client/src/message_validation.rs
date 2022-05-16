use crate::types::{Capability, ConfigWaypoint, Datastore};
use anyhow::{bail, Result};
use url::Url;

/// Check whether `server_capabilities` include URL capability with the same scheme value,
/// as the input `url` parameter.
pub fn validate_url(url: &str, server_capabilities: &[Capability]) -> Result<()> {
    for cap in server_capabilities.iter() {
        if let Capability::Url(schemes) = cap {
            let wp_url = Url::parse(url)?;
            if schemes.iter().any(|x| x == wp_url.scheme()) {
                return Ok(());
            } else {
                bail!(
                    "Server URL capability schemes: [{}] do not include waypoint scheme \"{}\".",
                    schemes.join(", "),
                    wp_url.scheme()
                );
            }
        }
    }
    bail!("Missing capability \":url\"");
}

/// For the input "waypoint", when it is of the "URL" type,
/// check whether `server_capabilities` include URL capability with the same scheme value.
/// For other waypoint enum types, returns Ok(()) irrespective of their value/payload.
pub fn validate_waypoint_url(
    waypoint: &ConfigWaypoint,
    server_capabilities: &[Capability],
) -> Result<()> {
    match waypoint {
        ConfigWaypoint::Datastore(_) => Ok(()),
        ConfigWaypoint::Url(url) => validate_url(url, server_capabilities),
    }
}

/// Validate that if and only if the specified `datastore` is equal to `checked_value`,
/// the `needed_capability` is present among the `server_capabilities`.
pub fn validate_datastore_capability(
    datastore: &Datastore,
    checked_value: &Datastore,
    needed_capability: &Capability,
    server_capabilities: &[Capability],
) -> Result<()> {
    if datastore == checked_value && !server_capabilities.contains(needed_capability) {
        bail!("Cannot use datastore \"{checked_value}\", missing capability {needed_capability}");
    }
    Ok(())
}

/// Validate that the specified `needed_capability` is present among the `server_capabilities`.
pub fn validate_capability_presence(
    needed_capability: &Capability,
    server_capabilities: &[Capability],
    extra_bail_info: &str,
) -> anyhow::Result<()> {
    if !server_capabilities.contains(needed_capability) {
        bail!("Missing required capability \"{needed_capability}\".{extra_bail_info}");
    }
    Ok(())
}
