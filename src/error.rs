use super::*;

use num_enum::TryFromPrimitiveError;
use std::{io, str::Utf8Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NinResError {
    #[error("Type unknown or not implemented. Magic number: {0:?}")]
    TypeUnknownOrNotImplemented([u8; 4]),
    #[error("IO error: {0}")]
    IoError(std::io::Error),
    #[error("Byte order invalid")]
    ByteOrderInvalid,
    #[error("UTF8 encoding error: {0}")]
    Utf8(Utf8Error),
    #[cfg(feature = "tar_ninres")]
    #[error("Tar append error")]
    TarAppend,
    #[cfg(feature = "zstd")]
    #[error("ZSTD error: {0}")]
    ZstdError(String),
}

impl<'a> From<io::Error> for NinResError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl<'a> From<TryFromPrimitiveError<ByteOrder>> for NinResError {
    fn from(_: TryFromPrimitiveError<ByteOrder>) -> Self {
        Self::ByteOrderInvalid
    }
}

impl<'a> From<FromUtf8Error> for NinResError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8(err.utf8_error())
    }
}

impl<'a> From<Utf8Error> for NinResError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8(err)
    }
}
