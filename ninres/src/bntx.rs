use crate::{ByteOrderMark, Error};

use std::{collections::HashMap, convert::TryFrom, io::SeekFrom};

#[derive(Clone, Debug)]
pub struct BNTX {
    pub header: BNTXHeader,
    texture_count: i32,
    texture_array_offset: i64,
    texture_data_offset: i64,
    texture_dict_offset: i64,
    string_table_entries: HashMap<u64, StringTableEntry>,
    textures: Vec<Texture>,
}

#[derive(Clone, Debug)]
pub struct BNTXHeader {
    alignment: u8,
    target_address_size: u8,
    file_name_offset: u32,
    flag: u16,
    block_offset: u16,
    relocation_table_offset: u32,
    file_size: u32,
}

#[derive(Clone, Debug)]
pub struct StringTableEntry {
    size: u16,
    string: String,
}

#[derive(Clone, Debug)]
pub struct Texture {
    flags: u8,
    dim: u8,
    tile_mode: u16,
    swizzle: u16,
    mip_count: u16,
    sample_count: u16,
    format: u32,
    access_flags: u32,
    width: u32,
    height: u32,
    depth: u32,
    array_length: u32,
    texture_layout: u32,
    texture_layout2: u32,
    image_size: u32,
    alignment: u32,
    channel_type: u32,
    surface_dim: u8,
    name: String,
}

impl BNTX {
    pub fn try_new(buffer: &[u8]) -> Result<Self, Error> {
        let mut bom = ByteOrderMark::try_new(
            buffer.to_vec(),
            u16::from_be_bytes([buffer[0xC], buffer[0xD]]),
        )?;
        let alignment = buffer[0xE];
        let target_address_size = buffer[0xF];
        bom.set_position(0x10);
        let file_name_offset = bom.read_u32()?;
        let flag = bom.read_u16()?;
        let block_offset = bom.read_u16()?;
        let relocation_table_offset = bom.read_u32()?;
        let file_size = bom.read_u32()?;

        bom.seek(SeekFrom::Current(4))?;
        let texture_count = bom.read_i32()?;
        let texture_array_offset = bom.read_i64()?;
        let texture_data_offset = bom.read_i64()?;
        let texture_dict_offset = bom.read_i64()?;

        bom.set_position(block_offset as u64 + 0x18);
        let mut string_table_entries = HashMap::with_capacity(texture_count as usize);
        for _ in 0..texture_count {
            let offset = bom.position();
            let size = bom.read_u16()?;
            let string = std::str::from_utf8(
                &buffer[bom.position() as usize..(bom.position() + size as u64) as usize],
            )?
            .to_string();
            string_table_entries.insert(offset, StringTableEntry { size, string });
            bom.seek(SeekFrom::Current(size as i64))?;
            if bom.position() % 2 == 1 {
                bom.seek(SeekFrom::Current(1))?;
            } else {
                bom.seek(SeekFrom::Current(2))?;
            }
        }

        let mut textures = Vec::with_capacity(texture_count as usize);
        for i in 0..texture_count {
            bom.set_position(texture_array_offset as u64 + i as u64 * 8);
            let pos = bom.read_i64()?;
            if &<[u8; 4]>::try_from(&buffer[pos as usize..pos as usize + 4]).unwrap() != b"BRTI" {
                return Err(Error::CorruptData);
            }
            bom.set_position(pos as u64 + 0x10);
            let flags = bom.read_u8()?;
            let dim = bom.read_u8()?;
            let tile_mode = bom.read_u16()?;
            let swizzle = bom.read_u16()?;
            let mip_count = bom.read_u16()?;
            let sample_count = bom.read_u16()?;
            bom.seek(SeekFrom::Current(2))?;
            let format = bom.read_u32()?;

            let access_flags = bom.read_u32()?;
            let width = bom.read_u32()?;
            let height = bom.read_u32()?;
            let depth = bom.read_u32()?;
            let array_length = bom.read_u32()?;
            let texture_layout = bom.read_u32()?;
            let texture_layout2 = bom.read_u32()?;
            bom.seek(SeekFrom::Current(20))?;
            let image_size = bom.read_u32()?;

            let alignment = bom.read_u32()?;
            let channel_type = bom.read_u32()?;
            let surface_dim = bom.read_u8()?;
            bom.seek(SeekFrom::Current(3))?;
            let name_offset = bom.read_u64()?;
            let name = string_table_entries
                .get(&name_offset)
                .map_or_else(|| Err(Error::CorruptData), Ok)?
                .string
                .clone();

            textures.push(Texture {
                flags,
                dim,
                tile_mode,
                swizzle,
                mip_count,
                sample_count,
                format,
                access_flags,
                width,
                height,
                depth,
                array_length,
                texture_layout,
                texture_layout2,
                image_size,
                alignment,
                channel_type,
                surface_dim,
                name,
            });

            // long ParentOffset = loader.ReadInt64();
            // long PtrOffset = loader.ReadInt64();
            // long UserDataOffset = loader.ReadInt64();
            // long TexPtr = loader.ReadInt64();
            // long TexView = loader.ReadInt64();
            // long descSlotDataOffset = loader.ReadInt64();
            // UserDataDict = loader.LoadDict();
        }

        let header = BNTXHeader {
            alignment,
            target_address_size,
            file_name_offset,
            flag,
            block_offset,
            relocation_table_offset,
            file_size,
        };
        Ok(Self {
            header,
            texture_count,
            texture_array_offset,
            texture_data_offset,
            texture_dict_offset,
            string_table_entries,
            textures,
        })
    }
}
