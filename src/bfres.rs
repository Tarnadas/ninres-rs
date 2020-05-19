//! Reads BFRES files.
//!
//! See http://mk8.tockdom.com/wiki/BFRES_(File_Format)

use crate::{ByteOrder, Error};

#[derive(Clone, Debug)]
pub struct Bfres {
    header: BfresHeader,
}

#[derive(Clone, Debug)]
pub struct BfresHeader {
    version_number: u16,
    byte_order: ByteOrder,
    file_length: u32,
    file_alignment: u32,
    file_name_offset: i32,
    string_table_length: i32,
    string_table_offset: i32,
    file_offsets: [i32; 12],
    file_counts: [u16; 12],
}

impl Bfres {
    pub fn new(buffer: &[u8]) -> Result<Bfres, Error> {
        Err(Error::TypeUnknownOrNotImplemented([
            buffer[0], buffer[1], buffer[2], buffer[3],
        ]))
    }
}
