use super::*;

use num_enum::TryFromPrimitiveError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum SarcError {
    ByteOrderInvalid,
    MagicInvalid,
    Utf8,
    #[cfg(feature = "tar")]
    TarAppend,
    #[cfg(feature = "tar")]
    IoError(std::io::Error),
}

impl From<TryFromPrimitiveError<ByteOrder>> for SarcError {
    fn from(_: TryFromPrimitiveError<ByteOrder>) -> Self {
        Self::ByteOrderInvalid
    }
}

impl From<FromUtf8Error> for SarcError {
    fn from(_: FromUtf8Error) -> Self {
        Self::Utf8
    }
}

impl From<std::io::Error> for SarcError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
