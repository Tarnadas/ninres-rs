use crate::{Error, NinResError};

use byteorder::{ByteOrder, BE, LE};
use std::{
    fmt::Debug,
    io::{Cursor, Seek, SeekFrom},
};

#[derive(Clone)]
#[repr(u16)]
pub enum ByteOrderMark {
    BigEndian(Cursor<Vec<u8>>),
    LittleEndian(Cursor<Vec<u8>>),
}

impl Debug for ByteOrderMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BigEndian(_) => f.write_str("ByteOrderMark::BigEndian"),
            Self::LittleEndian(_) => f.write_str("ByteOrderMark::LittleEndian"),
        }
    }
}

#[cfg(any(feature = "bfres", feature = "sarc"))]
impl ByteOrderMark {
    pub fn try_new(buffer: Vec<u8>, bom: u16) -> Result<Self, Error> {
        match bom {
            0xfeff => Ok(Self::BigEndian(Cursor::new(buffer))),
            0xfffe => Ok(Self::LittleEndian(Cursor::new(buffer))),
            _ => Err(NinResError::ByteOrderInvalid),
        }
    }
}

pub trait Buf {
    fn set_position(&mut self, pos: u64);

    fn seek(&mut self, seek_from: SeekFrom) -> Result<u64, Error>;

    fn read_u16(&mut self) -> Result<u16, Error>;

    fn read_u32(&mut self) -> Result<u32, Error>;
}

macro_rules! get_number {
    ( $func:ident, $num:ty, $bytes:expr ) => {
        fn $func(&mut self) -> Result<$num, Error> {
            match self {
                Self::BigEndian(cursor) => {
                    let res = BE::$func(
                        &cursor.get_ref()
                            [cursor.position() as usize..(cursor.position() + $bytes) as usize],
                    );
                    cursor.seek(SeekFrom::Current($bytes))?;
                    Ok(res)
                }
                Self::LittleEndian(cursor) => {
                    let res = LE::$func(
                        &cursor.get_ref()
                            [cursor.position() as usize..(cursor.position() + $bytes) as usize],
                    );
                    cursor.seek(SeekFrom::Current($bytes))?;
                    Ok(res)
                }
            }
        }
    };
}

impl Buf for ByteOrderMark {
    fn set_position(&mut self, pos: u64) {
        match self {
            Self::BigEndian(bytes) | Self::LittleEndian(bytes) => bytes.set_position(pos),
        }
    }

    fn seek(&mut self, seek_from: SeekFrom) -> Result<u64, Error> {
        match self {
            ByteOrderMark::BigEndian(cursor) | ByteOrderMark::LittleEndian(cursor) => {
                Ok(cursor.seek(seek_from)?)
            }
        }
    }

    get_number!(read_u16, u16, 2);
    get_number!(read_u32, u32, 4);
}
