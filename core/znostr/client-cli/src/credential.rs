use std::fs;
use std::fs::File;
use std::path::Path;
use nostr_sdk::{Contact, Metadata};
use nostr_sdk::hashes::sha256::Hash;
use nostr_sdk::key::XOnlyPublicKey;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::error::Error;
use crate::relay::Relay;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub(crate) secret_key_bech32: String, // nsec1...// private key
    pub(crate) public_key_bech32: String, // npub1...
    pub(crate) relays: Vec<Relay>,
    pub(crate) metadata: Metadata,
    pub(crate) contacts: Vec<Contact>,
    pub(crate) subscribed_pubkeys: Vec<XOnlyPublicKey>,
    pub(crate) subscribed_authors: Vec<XOnlyPublicKey>,
    // todo: zzz subscribed_channels should be ChannelId's ?
    pub(crate) subscribed_channels: Vec<Hash>,
}

impl AsRef<Credentials> for Credentials {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::new()
    }
}

/// implementation of Credentials struct
impl Credentials {
    /// Default constructor
   pub fn new() -> Self {
        Self {
            secret_key_bech32: "".to_owned(),
            public_key_bech32: "".to_owned(),
            relays: Vec::new(),
            metadata: Metadata::new(),
            contacts: Vec::new(),
            subscribed_pubkeys: Vec::new(),
            subscribed_authors: Vec::new(),
            subscribed_channels: Vec::new(),
        }
    }

    // /// Default constructor
    // fn create(
    //     secret_key_bech32: String,
    //     public_key_bech32: String,
    //     relays: Vec<Url>,
    //     metadata: Metadata,
    // ) -> Self {
    //     Self {
    //         secret_key_bech32,
    //         public_key_bech32,
    //         relays,
    //         metadata,
    //     }
    // }

    /// Constructor for Credentials
    pub(crate) fn load(path: &Path) -> Result<Credentials, Error> {
        let reader = File::open(path)?;
        Credentials::set_permissions(&reader)?;
        let credentials: Credentials = serde_json::from_reader(reader)?;
        let mut credentialsfiltered = credentials.clone();
        credentialsfiltered.secret_key_bech32 = "***".to_string();
        info!("loaded credentials are: {:?}", credentialsfiltered);
        Ok(credentials)
    }

    /// Writing the credentials to a file
    pub(crate) fn save(&self, path: &Path) -> Result<(), Error> {
        fs::create_dir_all(path.parent().ok_or(Error::NoHomeDirectory)?)?;
        let writer = File::create(path)?;
        serde_json::to_writer_pretty(&writer, self)?;
        Credentials::set_permissions(&writer)?;
        Ok(())
    }

    #[cfg(unix)]
    fn set_permissions(file: &File) -> Result<(), Error> {
        use std::os::unix::fs::PermissionsExt;
        let perms = file.metadata()?.permissions();
        // is the file world-readable? if so, reset the permissions to 600
        if perms.mode() & 0o4 == 0o4 {
            file.set_permissions(fs::Permissions::from_mode(0o600))
                .unwrap();
        }
        Ok(())
    }

    #[cfg(not(unix))]
    fn set_permissions(file: &File) -> Result<(), Error> {
        Ok(())
    }
}
