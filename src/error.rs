// Error types
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AccessKeyError {
    #[error(transparent)]
    Base32DecodeError(#[from] data_encoding::DecodeError),

    #[error("invalid key length")]
    InvalidLength,

    #[error("unknown key prefix: {0}")]
    UnknownPrefix(String),
}
