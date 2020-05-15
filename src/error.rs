use super::*;

use num_enum::TryFromPrimitiveError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SarcError {
    #[error("Byte order invalid")]
    ByteOrderInvalid,
    #[error("Magic number invalid. This is not a SARC file")]
    MagicInvalid,
    #[error("UTF8 encoding error: {0}")]
    Utf8(FromUtf8Error),
    #[cfg(feature = "tar_sarc")]
    #[error("Byte order invalid")]
    TarAppend,
    #[cfg(feature = "tar_sarc")]
    #[error("IO error: {0}")]
    IoError(std::io::Error),
    #[cfg(feature = "zstd")]
    #[error("ZSTD error: {0}")]
    ZstdError(String),
}

impl From<TryFromPrimitiveError<ByteOrder>> for SarcError {
    fn from(_: TryFromPrimitiveError<ByteOrder>) -> Self {
        Self::ByteOrderInvalid
    }
}

impl From<FromUtf8Error> for SarcError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}

#[cfg(feature = "tar_sarc")]
impl From<std::io::Error> for SarcError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
