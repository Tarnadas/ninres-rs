use super::*;

use num_enum::TryFromPrimitiveError;
use std::{array::TryFromSliceError, str::Utf8Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NinResError {
    #[error("Type unknown or not implemented. Magic number: {0:?}")]
    TypeUnknownOrNotImplemented([u8; 4]),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Byte order invalid")]
    ByteOrderInvalid,
    #[error(transparent)]
    TryFromSlice(#[from] TryFromSliceError),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
    #[cfg(feature = "tar")]
    #[error("Tar append error")]
    TarAppend,
    #[cfg(feature = "zstd")]
    #[error("ZSTD error: {0}")]
    ZstdError(String),
}

impl<'a> From<TryFromPrimitiveError<ByteOrderMark>> for NinResError {
    fn from(_: TryFromPrimitiveError<ByteOrderMark>) -> Self {
        Self::ByteOrderInvalid
    }
}

impl<'a> From<FromUtf8Error> for NinResError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8(err.utf8_error())
    }
}
