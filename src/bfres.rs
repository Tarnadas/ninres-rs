//! Reads BFRES files.
//!
//! See http://mk8.tockdom.com/wiki/BFRES_(File_Format)

use crate::{read_i32, read_u16, read_u32, ByteOrderMask, Error};

use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug)]
pub struct Bfres {
    header: BfresHeader,
}

#[derive(Clone, Debug)]
pub struct BfresHeader {
    version_number: u32,
    bom: ByteOrderMask,
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
        let bom = ByteOrderMask::try_from(read_u16(buffer, 0x6, ByteOrderMask::BigEndian))?;
        let version_number = read_u32(buffer, 0x4, bom);
        let file_length = read_u32(buffer, 0xC, bom);
        let file_alignment = read_u32(buffer, 0x10, bom);
        let file_name_offset = read_i32(buffer, 0x14, bom);
        let string_table_length = read_i32(buffer, 0x18, bom);
        let string_table_offset = read_i32(buffer, 0x1C, bom);
        let file_offsets = (0..12)
            .map(|i| read_i32(buffer, 0x20 + 4 * i, bom))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()?;
        let file_counts = (0..12)
            .map(|i| read_u16(buffer, 0x50 + 2 * i, bom))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()?;

        Ok(Bfres {
            header: BfresHeader {
                version_number,
                bom,
                file_length,
                file_alignment,
                file_name_offset,
                string_table_length,
                string_table_offset,
                file_offsets,
                file_counts,
            },
        })
    }
}
