mod util;

use crate::{ByteOrderMark, Error};

#[cfg(target_arch = "wasm32")]
use js_sys::JsString;
use std::{cmp, collections::HashMap, convert::TryFrom, io::SeekFrom};
use util::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct BNTX {
    header: BNTXHeader,
    texture_count: i32,
    texture_array_offset: i64,
    texture_data_offset: i64,
    texture_dict_offset: i64,
    string_table_entries: HashMap<u64, StringTableEntry>,
    textures: Vec<Texture>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Texture {
    flags: u8,
    dim: u8,
    tile_mode: u16,
    swizzle: u16,
    mip_count: u16,
    sample_count: u16,
    format: u32,
    access_flags: u32,
    pub width: u32,
    pub height: u32,
    depth: u32,
    array_length: u32,
    texture_layout: u32,
    texture_layout2: u32,
    image_size: u32,
    alignment: u32,
    channel_type: u32,
    surface_dim: u8,
    name: String,
    parent_offset: u64,
    ptr_offset: u64,
    user_data_offset: u64,
    tex_ptr: u64,
    tex_view: u64,
    desc_slot_data_offset: u64,
    user_dict_offset: u64,
    mip_offsets: Vec<u64>,
    #[derivative(Debug = "ignore")]
    texture_data: Vec<Vec<Vec<u8>>>,
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

            let parent_offset = bom.read_u64()?;
            let ptr_offset = bom.read_u64()?;
            let user_data_offset = bom.read_u64()?;
            let tex_ptr = bom.read_u64()?;
            let tex_view = bom.read_u64()?;
            let desc_slot_data_offset = bom.read_u64()?;
            let user_dict_offset = bom.read_u64()?;

            let mut mip_offsets = Vec::with_capacity(mip_count as usize);
            bom.set_position(ptr_offset);
            let first_mip_offset = bom.read_u64()?;
            mip_offsets.push(0);
            for _ in 1..mip_count {
                mip_offsets.push(bom.read_u64()? - first_mip_offset);
            }

            let mut texture_data = Vec::with_capacity(array_length as usize);
            bom.set_position(first_mip_offset);

            let (blk_width, blk_height) =
                if let Some((w, h)) = BLK_DIMS.lock().unwrap().get(&(format >> 8)) {
                    (*w, *h)
                } else {
                    (1, 1)
                };
            let bpp = *BPPS.lock().unwrap().get(&(format >> 8)).unwrap();
            let target = true; // "NX "

            let block_height_log2 = texture_layout & 7;
            let lines_per_block_height = (1 << block_height_log2) * 8;
            let mut block_height_shift = 0;

            for _ in 0..array_length {
                let mut mips = Vec::with_capacity(mip_count as usize);
                for (mip_level, mip_offset) in mip_offsets.iter().enumerate() {
                    let size = (image_size as u64 - mip_offset) / array_length as u64;
                    bom.set_position(first_mip_offset + *mip_offset);
                    let buffer = (buffer
                        [bom.position() as usize..(bom.position() + size) as usize])
                        .to_vec();

                    let width = cmp::max(1, width >> mip_level);
                    let height = cmp::max(1, height >> mip_level);

                    let size =
                        div_round_up(width, blk_width) * div_round_up(height, blk_height) * bpp;

                    if pow2_round_up(div_round_up(height, blk_height)) < lines_per_block_height {
                        block_height_shift += 1;
                    }

                    let buffer = deswizzle(
                        width,
                        height,
                        blk_width,
                        blk_height,
                        target,
                        bpp,
                        tile_mode,
                        (block_height_log2)
                            .checked_sub(block_height_shift)
                            .unwrap_or_default(),
                        buffer,
                    )?;
                    mips.push(buffer[..size as usize].to_vec());
                }
                texture_data.push(mips);
            }

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
                parent_offset,
                ptr_offset,
                user_data_offset,
                tex_ptr,
                tex_view,
                desc_slot_data_offset,
                user_dict_offset,
                mip_offsets,
                texture_data,
            });
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_textures(&self) -> &Vec<Texture> {
        &self.textures
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BNTX {
    #[wasm_bindgen(js_name = getTextures)]
    pub fn get_textures(&self) -> Box<[JsValue]> {
        self.textures
            .clone()
            .into_iter()
            .map(|t| t.into())
            .collect()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Texture {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_texture_data(&self) -> &Vec<Vec<Vec<u8>>> {
        &self.texture_data
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Texture {
    #[wasm_bindgen(js_name = getName)]
    pub fn get_name(&self) -> JsString {
        self.name.clone().into()
    }

    #[wasm_bindgen(js_name = getTextureData)]
    pub fn get_texture_data(&self, tex_count: usize, mip_level: usize) -> Option<Box<[u8]>> {
        self.texture_data
            .get(tex_count)
            .map(|d| d.get(mip_level))
            .flatten()
            .cloned()
            .map(|d| {
                web_sys::console::log_1(&format!("{}", d.len()).into());
                d.into_boxed_slice()
            })
    }

    #[wasm_bindgen(js_name = getTexCount)]
    pub fn get_tex_count(&self) -> usize {
        self.texture_data.len()
    }

    #[wasm_bindgen(js_name = getMipLevel)]
    pub fn get_mip_level(&self, tex_count: usize) -> Option<usize> {
        self.texture_data.get(tex_count).map(|d| d.len())
    }

    #[cfg(feature = "png")]
    #[wasm_bindgen(js_name = asPng)]
    pub fn as_png(&self, tex_count: usize, mip_level: usize) -> Option<Box<[u8]>> {
        use image::{DynamicImage, ImageBuffer, ImageOutputFormat};

        let width = cmp::max(1, self.width >> mip_level);
        let height = cmp::max(1, self.height >> mip_level);
        if let Some(buf) = self
            .texture_data
            .get(tex_count)
            .map(|d| d.get(mip_level))
            .flatten()
        {
            if let Some(buf) = ImageBuffer::from_raw(width, height, buf.clone()) {
                let image = DynamicImage::ImageRgba8(buf);
                let mut res = vec![];
                if let Err(err) = image.write_to(&mut res, ImageOutputFormat::Png) {
                    web_sys::console::error_1(&format!("asPng threw an error: {}", err).into());
                    return None;
                }
                Some(res.into_boxed_slice())
            } else {
                None
            }
        } else {
            None
        }
    }
}
