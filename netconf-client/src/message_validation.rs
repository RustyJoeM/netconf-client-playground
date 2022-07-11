use crate::types::{Capability, Datastore};
use anyhow::{anyhow, bail, Result};
use url::Url;

/// Check whether `server_capabilities` include URL capability with the same scheme value,
/// as the input `url` parameter.
pub fn validate_url(url: &str, server_capabilities: &[Capability]) -> Result<()> {
    let wp_url = Url::parse(url)?;

    server_capabilities
        .iter()
        .filter_map(|c| match c {
            Capability::Url(schemes) => Some(schemes),
            _ => None,
        })
        .find(|schemes| schemes.iter().any(|x| x == wp_url.scheme()))
        .map(|_| ())
        .ok_or_else(|| anyhow!("Missing capability \":url\""))

    /* for cap in server_capabilities.iter() {
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
    } */
    /* bail!("Missing capability \":url\""); */
}

/// Validate that if and only if the specified `datastore` is equal to `trigger`,
/// the `needed_capability` is present among the `server_capabilities`.
pub fn validate_datastore_capability(
    datastore: &Datastore,
    trigger: &Datastore,
    needed_capability: &Capability,
    server_capabilities: &[Capability],
) -> Result<()> {
    if datastore == trigger && !server_capabilities.contains(needed_capability) {
        bail!("Cannot use datastore \"{trigger}\", missing capability {needed_capability}");
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
