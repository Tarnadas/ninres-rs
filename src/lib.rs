//! Read commonly used Nintendo file formats.
//!
//! Please refer to the Wiki:
//! https://github.com/Kinnay/Nintendo-File-Formats/wiki
//!
//! All file formats are behind feature flags.
//! Here is a list of available Nintendo file format features:
//!
//! `bfres`, `sarc`
//!
//! You can also enable additional features:
//!
//! `tar_ninres`: write Nintendo resource to tar ball.
//!
//! `zstd`: ZSTD decompression.
//!
//! All features of this crate can be compiled to WebAssembly.
//!
//! # Examples
//!
//! Enable desired features in `Cargo.toml`.
//!
//! ```toml
//!     [dependencies]
//!     ninres = { version = "*", features = ["bfres", "sarc", "zstd"] }
//! ```
//!
//! In your `main.rs`.
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

/// Smart convert buffer into any known Nintendo file format.
///
/// # Examples
///
/// ```
/// # use ninres::NinResResult;
/// # #[cfg(all(feature = "sarc", feature = "bfres"))]
/// # fn example() -> NinResResult {
///     use std::fs::read;
///     use ninres::{NinRes, NinResFile};
///
///     let buffer = read("foo.pack")?;
///     let ninres = buffer.as_ninres()?;
///     
///     match &ninres {
///        NinResFile::Bfres(_bfres) => {}
///        NinResFile::Sarc(_sarc) => {}
///     }
///
///     Ok(ninres)
/// # }
/// ```
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

/// Convert resource into tar buffer.
/// This buffer can then e.g. be stored in a file.
///
/// The `mode` parameter refers to the file mode within the tar ball.
///
/// # Examples
///
/// ```
/// # use ninres::NinResError;
/// #[cfg(all(not(target_arch = "wasm32"), feature = "sarc"))]
/// fn main() -> Result<(), NinResError> {
///     use ninres::{sarc::Sarc, IntoTar};
///     use std::{fs::{read, File}, io::Write};
///
///     let sarc_file = Sarc::new(&read("./assets/M1_Model.pack")?)?;
///     let tar = sarc_file.into_tar(0o644)?;
///
///     let mut file = File::create("M1_Model.tar")?;
///     file.write_all(&tar.into_inner()[..])?;
///     Ok(())
/// }
/// ```
#[cfg(feature = "tar_ninres")]
pub trait IntoTar {
    fn into_tar(self, mode: u32) -> Result<std::io::Cursor<Vec<u8>>, Error>;
}
