//! Read commonly used Nintendo file formats.
//!
//! Please refer to the Wiki:
//! https://github.com/Kinnay/Nintendo-File-Formats/wiki
//!
//! All file formats are behind feature flags.
//! Here is a list of available Nintendo resource feature:
//! ["bfres", "sarc"]
//!
//! You can also enable additional features:
//!
//! `tar_ninres`: write Nintendo resource to tar ball.
//!
//! `zstd`: ZSTD decompression.
//!
//! All features of this crate can be compiles to WebAssembly.
//!
//! # Examples
//!
//! ```
//! # use ninres::NinResResult;
//! # #[cfg(all(feature = "sarc", feature = "bfres"))]
//! # fn example() -> NinResResult {
//!     use std::fs::read;
//!     use ninres::{NinRes, NinResFile};
//!
//!     let buffer = read("foo.pack")?;
//!     let ninres = buffer.as_ninres()?;
//!     
//!     match &ninres {
//!        NinResFile::Bfres(_bfres) => {}
//!        NinResFile::Sarc(_sarc) => {}
//!     }
//!
//!     Ok(ninres)
//! # }
//! ```
//!

#[cfg(feature = "zstd")]
#[macro_use]
extern crate cfg_if;

mod error;

#[cfg(feature = "bfres")]
pub mod bfres;

#[cfg(feature = "sarc")]
pub mod sarc;

pub use error::NinResError;
use num_enum::TryFromPrimitive;

pub(crate) type Error = NinResError;
#[cfg(any(feature = "bfres", feature = "sarc"))]
pub type NinResResult = Result<NinResFile, Error>;

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum ByteOrder {
    BigEndian = 0xfeff,
    LittleEndian = 0xfffe,
}

#[cfg(any(feature = "bfres", feature = "sarc"))]
#[derive(Clone)]
pub enum NinResFile {
    #[cfg(feature = "bfres")]
    Bfres(bfres::Bfres),
    #[cfg(feature = "sarc")]
    Sarc(sarc::Sarc),
}

#[cfg(any(feature = "bfres", feature = "sarc"))]
pub trait NinRes {
    fn as_ninres(&self) -> NinResResult;
    fn into_ninres(self) -> NinResResult;
}

#[cfg(any(feature = "bfres", feature = "sarc"))]
impl NinRes for &[u8] {
    fn as_ninres(&self) -> NinResResult {
        match std::str::from_utf8(&self[..4])? {
            #[cfg(feature = "sarc")]
            "SARC" => Ok(NinResFile::Sarc(sarc::Sarc::new(self)?)),
            #[cfg(feature = "bfres")]
            "FRES" => Ok(NinResFile::Bfres(bfres::Bfres::new(self)?)),
            _ => Err(NinResError::TypeUnknownOrNotImplemented([
                self[0], self[1], self[2], self[3],
            ])),
        }
    }

    fn into_ninres(self) -> NinResResult {
        match std::str::from_utf8(&self[..4])? {
            #[cfg(feature = "sarc")]
            "SARC" => Ok(NinResFile::Sarc(sarc::Sarc::new(self)?)),
            #[cfg(feature = "bfres")]
            "FRES" => Ok(NinResFile::Bfres(bfres::Bfres::new(self)?)),
            _ => Err(NinResError::TypeUnknownOrNotImplemented([
                self[0], self[1], self[2], self[3],
            ])),
        }
    }
}

#[cfg(any(feature = "bfres", feature = "sarc"))]
impl NinRes for Vec<u8> {
    fn as_ninres(&self) -> NinResResult {
        match std::str::from_utf8(&self[..4])? {
            #[cfg(feature = "sarc")]
            "SARC" => Ok(NinResFile::Sarc(sarc::Sarc::new(self)?)),
            #[cfg(feature = "bfres")]
            "FRES" => Ok(NinResFile::Bfres(bfres::Bfres::new(self)?)),
            _ => Err(NinResError::TypeUnknownOrNotImplemented([
                self[0], self[1], self[2], self[3],
            ])),
        }
    }

    fn into_ninres(self) -> NinResResult {
        match std::str::from_utf8(&self[..4])? {
            #[cfg(feature = "sarc")]
            "SARC" => Ok(NinResFile::Sarc(sarc::Sarc::new(&self[..])?)),
            #[cfg(feature = "bfres")]
            "FRES" => Ok(NinResFile::Bfres(bfres::Bfres::new(&self[..])?)),
            _ => Err(NinResError::TypeUnknownOrNotImplemented([
                self[0], self[1], self[2], self[3],
            ])),
        }
    }
}
