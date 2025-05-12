use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(&'static str),

    #[error("No valid home directory path")]
    NoHomeDirectory,

    #[error("Failure with key")]
    KeyFailure,

    #[error("User Already Exists")]
    UserAlreadyExists,

    #[error("Failure while storing data")]
    StorageFailure,

    #[error("Invalid File")]
    InvalidFile,

    #[error("Creating User Failed")]
    CreatingUserFailed,

    #[error("Publish Poll Failed")]
    PublishPollFailed,

    #[error("Make Vote Failed")]
    MakeVoteFailed,



    #[error("Reading Credentials Failed")]
    ReadingCredentialsFailed,

    #[error("Cannot Connect To Relays")]
    CannotConnectToRelays,

    #[error("Add Relay Failed")]
    AddRelayFailed,

    #[error("Conversion Failed")]
    ConversionFailed,

    #[error("Publish Failed")]
    PublishFailed,

    #[error("Publish POW Failed")]
    PublishPowFailed,

    #[error("DM Failed")]
    DmFailed,

    #[error("Send Failed")]
    SendFailed,

    #[error("Send Channel Failed")]
    SendChannelFailed,

    #[error("Listen Failed")]
    ListenFailed,

    #[error("Subscription Failed")]
    SubscriptionFailed,

    #[error("Unsubscribe Failed")]
    UnsubscribeFailed,

    #[error("Get Entity Failed")]
    GetEntityFailed,

    #[error("Invalid Client Connection")]
    InvalidClientConnection,

    #[error("Invalid Key")]
    InvalidKey,

    #[error("Invalid Hash")]
    InvalidHash,

    #[error("Unknown CLI parameter")]
    UnknownCliParameter,

    #[error("Unsupported CLI parameter")]
    UnsupportedCliParameter,

    #[error("Missing User")]
    MissingUser,

    #[error("Missing Password")]
    MissingPassword,

    #[error("Missing CLI parameter")]
    MissingCliParameter,

    #[error("Not Implemented Yet")]
    NotImplementedYet,

    #[error("No Credentials Found")]
    NoCredentialsFound,

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    NostrNip04(#[from] nostr_sdk::nostr::nips::nip04::Error),

    #[error(transparent)]
    NostrKey(#[from] nostr_sdk::nostr::key::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

/// Function to create custom error messaages on the fly with static text
#[allow(dead_code)]
impl Error {
    pub(crate) fn custom<T>(message: &'static str) -> Result<T, Error> {
        Err(Error::Custom(message))
    }
}
