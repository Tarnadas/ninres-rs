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

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(any(feature = "bfres", feature = "sarc", feature = "tar"))]
pub(crate) type Error = NinResError;
#[cfg(any(feature = "bfres", feature = "sarc"))]
pub type NinResResult = Result<NinResFile, Error>;

#[cfg(any(feature = "bfres", feature = "sarc"))]
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
pub enum NinResFile {
    #[cfg(feature = "bfres")]
    Bfres(bfres::Bfres),
    #[cfg(feature = "sarc")]
    Sarc(sarc::Sarc),
}

#[cfg(any(feature = "bfres", feature = "sarc"))]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub enum NinResFile {
    #[cfg(feature = "bfres")]
    Bfres,
    #[cfg(feature = "sarc")]
    Sarc,
}

#[cfg(any(feature = "bfres", feature = "sarc"))]
#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct NinResFileExt {
    file_type: NinResFile,
    sarc: Option<Sarc>,
    bfres: Option<Bfres>,
}

#[wasm_bindgen]
impl NinResFileExt {
    #[wasm_bindgen(js_name = getFileType)]
    pub fn get_file_type(&self) -> NinResFile {
        self.file_type.clone()
    }

    #[wasm_bindgen(js_name = getSarc)]
    pub fn get_sarc(&self) -> Option<Sarc> {
        self.sarc.clone()
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
#[cfg(not(target_arch = "wasm32"))]
pub trait NinRes {
    fn as_ninres(&self) -> NinResResult;
    fn into_ninres(self) -> NinResResult;
}

#[cfg(any(feature = "bfres", feature = "sarc"))]
#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(any(feature = "bfres", feature = "sarc"))]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NinResFileExt {
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(buf: &[u8]) -> Result<NinResFileExt, JsValue> {
        match std::str::from_utf8(&buf[..4]).map_err(|err| JsValue::from(format!("{}", err)))? {
            #[cfg(feature = "sarc")]
            "SARC" => Ok(NinResFileExt {
                file_type: NinResFile::Sarc,
                sarc: Some(Sarc::new(buf)?),
                bfres: None,
            }),
            #[cfg(feature = "bfres")]
            "FRES" => Ok(NinResFileExt {
                file_type: NinResFile::Bfres,
                sarc: None,
                bfres: Some(Bfres::new(buf)?),
            }),
            _ => Err(
                NinResError::TypeUnknownOrNotImplemented([buf[0], buf[1], buf[2], buf[3]]).into(),
            ),
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

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        /// Setup panic hook for WebAssembly calls.
        /// This will forward Rust panics to console.error
        #[wasm_bindgen(js_name = setupPanicHook)]
        pub fn setup_panic_hook() {
            console_error_panic_hook::set_once();
        }

        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
