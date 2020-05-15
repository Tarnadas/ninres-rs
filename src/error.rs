use super::*;

use num_enum::TryFromPrimitiveError;
use std::{str::Utf8Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SarcError<'a> {
    #[error("Byte order invalid")]
    ByteOrderInvalid,
    #[error("Magic number invalid: {0:?}. This is not a SARC file")]
    MagicInvalid(&'a [u8]),
    #[error("UTF8 encoding error: {0}")]
    Utf8(Utf8Error),
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

impl<'a> From<TryFromPrimitiveError<ByteOrder>> for SarcError<'a> {
    fn from(_: TryFromPrimitiveError<ByteOrder>) -> Self {
        Self::ByteOrderInvalid
    }
}

impl<'a> From<FromUtf8Error> for SarcError<'a> {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8(err.utf8_error())
    }
}

impl<'a> From<Utf8Error> for SarcError<'a> {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8(err)
    }
}

#[cfg(feature = "tar_sarc")]
impl<'a> From<std::io::Error> for SarcError<'a> {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
