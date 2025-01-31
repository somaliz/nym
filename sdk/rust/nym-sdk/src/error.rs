use std::path::PathBuf;

/// Top-level Error enum for the mixnet client and its relevant types.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("toml serialization error: {0}")]
    TomlSerializationError(#[from] toml::ser::Error),

    #[error("toml deserialization error: {0}")]
    TomlDeserializationError(#[from] toml::de::Error),

    #[error("Ed25519 error: {0}")]
    Ed25519RecoveryError(#[from] nym_crypto::asymmetric::identity::Ed25519RecoveryError),

    #[error(transparent)]
    ClientCoreError(#[from] nym_client_core::error::ClientCoreError),

    #[error("key file encountered that we don't want to overwrite: {0}")]
    DontOverwrite(PathBuf),

    #[error("shared gateway key file encountered that we don't want to overwrite: {0}")]
    DontOverwriteGatewayKey(PathBuf),

    #[error("no gateway config available for writing")]
    GatewayNotAvailableForWriting,

    #[error("expected to received a directory, received: {0}")]
    ExpectedDirectory(PathBuf),

    #[error("failed to transition to registered state before connection to mixnet")]
    FailedToTransitionToRegisteredState,

    #[error(
        "registering with gateway when the client is already in a registered state is not \
         supported, and likely and user mistake"
    )]
    ReregisteringGatewayNotSupported,

    #[error("no gateway key set")]
    NoGatewayKeySet,

    #[error("credentials mode not enabled")]
    DisabledCredentialsMode,

    #[error("bad validator details: {0}")]
    BadValidatorDetails(#[from] nym_validator_client::ValidatorClientError),

    #[error("socks5 configuration set: {}, but expected to be {}", set, !set)]
    Socks5Config { set: bool },

    #[error("socks5 channel could not be started")]
    Socks5NotStarted,

    #[error(
        "deposited funds were not converted to a deposit - {reason}; the voucher blob can be used for \
    later retry"
    )]
    UnconvertedDeposit {
        reason: nym_bandwidth_controller::error::BandwidthControllerError,
        voucher_blob: crate::bandwidth::VoucherBlob,
    },

    #[error("bandwidth controller error: {0}")]
    BandwidthControllerError(#[from] nym_bandwidth_controller::error::BandwidthControllerError),

    #[error("invalid voucher blob")]
    InvalidVoucherBlob,

    #[error("invalid mnemonic: {0}")]
    InvalidMnemonic(#[from] bip39::Error),

    #[error("failed to create reply storage backend: {source}")]
    StorageError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
