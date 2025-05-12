use std::net::SocketAddr;
use nostr_sdk::Url;
use serde::{Deserialize, Serialize};

/// A struct for the relays. These will be serialized into JSON
/// and written to the credentials.json file for permanent storage and
/// future access.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Relay {
    pub(crate) url: Url,
    pub(crate) proxy: Option<SocketAddr>,
}

impl AsRef<Relay> for Relay {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Default for Relay {
    fn default() -> Self {
        Self::new(Url::parse("wss://relay.nostr.info/").unwrap(), None)
    }
}

/// implementation of Relay struct
impl Relay {
    /// Default constructor
    pub(crate) fn new(url: Url, proxy: Option<SocketAddr>) -> Self {
        Self { url, proxy }
    }
}