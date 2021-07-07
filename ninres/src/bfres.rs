//! Reads BFRES files.
//!
//! See http://mk8.tockdom.com/wiki/BFRES_(File_Format)

use crate::{ByteOrderMark, Error, BNTX};

use std::io::SeekFrom;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct Bfres {
    version_number: u32,
    bom: ByteOrderMark,
    byte_alignment: u8,
    file_name_offset: u32,
    flags: u16,
    block_offset: u16,
    relocation_table_offset: u32,
    bfres_size: u32,
    file_name_length_offset: u64,
    embedded_files_offset: u64,
    embedded_files_dictionary_offset: u64,
    embedded_files_data_offset: u64,
    embedded_files_data_size: u64,
    embedded_files_count: u32,
    string_table_offset: u64,
    string_table_size: u32,
    embedded_files: Vec<EmbeddedFile>,
}

impl Bfres {
    pub fn new(buffer: &[u8]) -> Result<Bfres, Error> {
        let mut bom = ByteOrderMark::try_new(
            buffer.to_vec(),
            u16::from_be_bytes([buffer[0xC], buffer[0xD]]),
        )?;
        bom.set_position(8);
        let version_number = bom.read_u32()?;
        let byte_alignment = buffer[0xE];
        bom.seek(SeekFrom::Current(4))?;
        let file_name_offset = bom.read_u32()?;
        let flags = bom.read_u16()?;
        let block_offset = bom.read_u16()?;
        let relocation_table_offset = bom.read_u32()?;
        let bfres_size = bom.read_u32()?;
        let file_name_length_offset = bom.read_u64()?;

        bom.set_position(0xB8);
        let embedded_files_offset = bom.read_u64()?;
        let embedded_files_dictionary_offset = bom.read_u64()?;

        bom.seek(SeekFrom::Current(8))?;
        let string_table_offset = bom.read_u64()?;
        let string_table_size = bom.read_u32()?;

        bom.set_position(embedded_files_offset);
        let embedded_files_data_offset = bom.read_u64()?;
        let embedded_files_data_size = bom.read_u64()?;
        bom.set_position(embedded_files_dictionary_offset + 4);
        let embedded_files_count = bom.read_u32()?;

        let mut embedded_files = vec![];
        for n in 0..embedded_files_count {
            let offset = embedded_files_data_offset + n as u64 * embedded_files_data_size;
            let data = &buffer[offset as usize..(offset + embedded_files_data_size) as usize];

            let file = match std::str::from_utf8(&data[..4])? {
                "BNTX" => EmbeddedFile::BNTX(BNTX::try_new(data)?),
                _ => continue,
            };

            embedded_files.push(file)
        }

        Ok(Bfres {
            version_number,
            bom,
            byte_alignment,
            file_name_offset,
            flags,
            block_offset,
            relocation_table_offset,
            bfres_size,
            file_name_length_offset,
            embedded_files_offset,
            embedded_files_dictionary_offset,
            embedded_files_data_offset,
            embedded_files_data_size,
            embedded_files_count,
            string_table_offset,
            string_table_size,
            embedded_files,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_embedded_files(&self) -> &Vec<EmbeddedFile> {
        &self.embedded_files
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Bfres {
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(buf: &[u8]) -> Result<Bfres, JsValue> {
        Ok(Bfres::new(buf)?)
    }

    #[wasm_bindgen(js_name = intoBntxFiles)]
    pub fn into_bntx_files(self) -> Box<[JsValue]> {
        self.embedded_files
            .into_iter()
            .map(|t| match t {
                EmbeddedFile::BNTX(bntx) => bntx.into(),
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub enum EmbeddedFile {
    BNTX(BNTX),
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static M1_PLAYER_MARIOMDL: &[u8] = include_bytes!("../../assets/M1_Player_MarioMdl.bfres");

    #[test_case(M1_PLAYER_MARIOMDL; "with M1 Player MarioMdl")]
    fn test_read_bfres(bfres_file: &[u8]) {
        let bfres_file = Bfres::new(bfres_file);
        dbg!(bfres_file.as_ref().unwrap());

        // assert!(bfres_file.is_ok());
    }
}
