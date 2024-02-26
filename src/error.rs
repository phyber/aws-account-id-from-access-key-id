// Error types
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AccessKeyError {
    /// Returned if the base32 cannot be decoded for some reason.
    #[error(transparent)]
    Base32DecodeError(#[from] data_encoding::DecodeError),

    /// Returned if the access key has an invalid length.
    #[error("invalid key length")]
    InvalidLength,

    /// Returned if the prefix of the access key is unknown.
    #[error("unknown key prefix: {0}")]
    UnknownPrefix(String),
}
