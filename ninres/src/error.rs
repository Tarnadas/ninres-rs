use super::*;

use num_enum::TryFromPrimitiveError;
use std::{array::TryFromSliceError, io, str::Utf8Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NinResError {
    #[error("Type unknown or not implemented. Magic number: {0:?}")]
    TypeUnknownOrNotImplemented([u8; 4]),
    #[error(transparent)]
    IoError(std::io::Error),
    #[error("Byte order invalid")]
    ByteOrderInvalid,
    #[error(transparent)]
    TryFromSlice(TryFromSliceError),
    #[error(transparent)]
    Utf8(Utf8Error),
    #[cfg(feature = "tar")]
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

impl<'a> From<TryFromPrimitiveError<ByteOrderMask>> for NinResError {
    fn from(_: TryFromPrimitiveError<ByteOrderMask>) -> Self {
        Self::ByteOrderInvalid
    }
}

impl<'a> From<TryFromSliceError> for NinResError {
    fn from(err: TryFromSliceError) -> Self {
        Self::TryFromSlice(err)
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
