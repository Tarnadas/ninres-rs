use super::*;

use num_enum::TryFromPrimitiveError;

#[derive(Debug)]
pub enum SarcError {
    ByteOrderInvalid,
    MagicInvalid,
}

impl From<TryFromPrimitiveError<ByteOrder>> for SarcError {
    fn from(_: TryFromPrimitiveError<ByteOrder>) -> Self {
        Self::ByteOrderInvalid
    }
}
