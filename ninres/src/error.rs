use std::{array::TryFromSliceError, str::Utf8Error, string::FromUtf8Error};
use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Error)]
pub enum NinResError {
    #[error("Type unknown or not implemented. Magic number: {0:?}")]
    TypeUnknownOrNotImplemented([u8; 4]),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Byte order invalid")]
    ByteOrderInvalid,
    #[error("CorruptData")]
    CorruptData,
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

impl<'a> From<FromUtf8Error> for NinResError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8(err.utf8_error())
    }
}

#[cfg(target_arch = "wasm32")]
impl From<NinResError> for JsValue {
    fn from(err: NinResError) -> JsValue {
        JsValue::from(format!("{}", err))
    }
}
