mod common;

/// All the data common types used by multiple various requests.
pub mod types;

/// All the requests grouped into sub-modules by NETCONF operation.
/// TODO - should not be used directly for communication,
/// unless (de)serializing only?
pub mod messages;

mod ssh_client;
pub use ssh_client::SshAuthentication;

mod netconf_session;
pub use netconf_session::NetconfSession;
