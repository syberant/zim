use bitreader;
use std;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown compression")]
    UnknownCompression,
    #[error("unknown mimetype")]
    UnknownMimeType,
    #[error("invalid magic number")]
    InvalidMagicNumber,
    #[error("invalid major version, must be 5 or 6")]
    InvalidVersion,
    #[error("invalid header")]
    InvalidHeader,
    #[error("invalid namespace")]
    InvalidNamespace,
    #[error("cluster extension requires major version 6")]
    InvalidClusterExtension,
    #[error("cluster is missing a blob list")]
    MissingBlobList,
    #[error("missing checksum")]
    MissingChecksum,
    #[error("invalid checksum")]
    InvalidChecksum,
    #[error("out of bounds access")]
    OutOfBounds,
    #[error("failed to parse: {0}")]
    ParsingError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<bitreader::BitReaderError> for Error {
    fn from(err: bitreader::BitReaderError) -> Error {
        Error::ParsingError(err.into())
    }
}
