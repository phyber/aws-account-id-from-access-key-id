// Error types
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AccessKeyError {
    /// Returned if the base32 cannot be decoded for some reason.
    #[error(transparent)]
    Base32DecodeError(#[from] data_encoding::DecodeError),

    /// Returned if the access key has an invalid length.
    #[error("invalid key length, account ids should be 20 characters")]
    InvalidLength,

    /// Returned if the prefix of the access key is unknown.
    #[error("unknown account id prefix: {0}")]
    UnknownPrefix(String),
}
