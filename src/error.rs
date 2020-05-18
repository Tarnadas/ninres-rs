use crate::sarc::SarcError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NinResError<'a> {
    #[error("SARC Error: {0}")]
    Sarc(SarcError<'a>),
}
