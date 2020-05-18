#[cfg(feature = "zstd")]
#[macro_use]
extern crate cfg_if;

mod error;
#[cfg(feature = "sarc")]
pub mod sarc;

pub use error::NinResError;
