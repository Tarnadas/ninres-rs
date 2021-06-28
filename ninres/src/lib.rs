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
//! `tar`: write Nintendo resource to tar ball.
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
//! # #[cfg(all(feature = "sarc", feature = "bfres"))]
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
//!         NinResFile::Bfres(_bfres) => {}
//!         NinResFile::Sarc(_sarc) => {}
//!     }
//!
//!     Ok(ninres)
//! # }
//! ```
//!

extern crate tar_crate as tar;

#[cfg(feature = "tar")]
#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate derivative;

mod bom;
mod error;

#[cfg(feature = "bfres")]
pub mod bfres;

// TODO feature flag
pub mod bntx;

#[cfg(feature = "sarc")]
pub mod sarc;

#[cfg(feature = "bfres")]
pub use bfres::*;
pub use bntx::*;
pub use bom::ByteOrderMark;
pub use error::NinResError;
#[cfg(feature = "sarc")]
pub use sarc::*;

#[cfg(any(feature = "bfres", feature = "sarc", feature = "tar"))]
pub(crate) type Error = NinResError;
#[cfg(any(feature = "bfres", feature = "sarc"))]
pub type NinResResult = Result<NinResFile, Error>;

#[cfg(any(feature = "bfres", feature = "sarc"))]
#[derive(Clone, Debug)]
pub enum NinResFile {
    #[cfg(feature = "bfres")]
    Bfres(bfres::Bfres),
    #[cfg(feature = "sarc")]
    Sarc(sarc::Sarc),
}

#[cfg(any(feature = "bfres", feature = "sarc"))]
impl NinResFile {
    pub fn get_extension(&self) -> &str {
        match self {
            #[cfg(feature = "bfres")]
            Self::Bfres(_) => "bfres",
            #[cfg(feature = "sarc")]
            Self::Sarc(_) => "sarc",
        }
    }
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
            "SARC" => Ok(NinResFile::Sarc(Sarc::new(self)?)),
            #[cfg(feature = "bfres")]
            "FRES" => Ok(NinResFile::Bfres(Bfres::new(self)?)),
            _ => Err(NinResError::TypeUnknownOrNotImplemented([
                self[0], self[1], self[2], self[3],
            ])),
        }
    }

    fn into_ninres(self) -> NinResResult {
        match std::str::from_utf8(&self[..4])? {
            #[cfg(feature = "sarc")]
            "SARC" => Ok(NinResFile::Sarc(Sarc::new(self)?)),
            #[cfg(feature = "bfres")]
            "FRES" => Ok(NinResFile::Bfres(Bfres::new(self)?)),
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
            "SARC" => Ok(NinResFile::Sarc(Sarc::new(self)?)),
            #[cfg(feature = "bfres")]
            "FRES" => Ok(NinResFile::Bfres(Bfres::new(self)?)),
            _ => Err(NinResError::TypeUnknownOrNotImplemented([
                self[0], self[1], self[2], self[3],
            ])),
        }
    }

    fn into_ninres(self) -> NinResResult {
        match std::str::from_utf8(&self[..4])? {
            #[cfg(feature = "sarc")]
            "SARC" => Ok(NinResFile::Sarc(Sarc::new(&self[..])?)),
            #[cfg(feature = "bfres")]
            "FRES" => Ok(NinResFile::Bfres(Bfres::new(&self[..])?)),
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
///     let sarc_file = Sarc::new(&read("../assets/M1_Model.pack")?)?;
///     let tar = sarc_file.into_tar(0o644)?;
///
///     let mut file = File::create("M1_Model.tar")?;
///     file.write_all(&tar.into_inner()[..])?;
///     Ok(())
/// }
/// ```
#[cfg(feature = "tar")]
pub trait IntoTar {
    fn into_tar(self, mode: u32) -> Result<std::io::Cursor<Vec<u8>>, Error>;
}
