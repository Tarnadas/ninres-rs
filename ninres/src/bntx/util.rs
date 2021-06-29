use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

use crate::Error;

pub static BLK_DIMS: Lazy<Mutex<HashMap<u32, (u32, u32)>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0x1a, (4, 4));
    map.insert(0x1b, (4, 4));
    map.insert(0x1c, (4, 4));
    map.insert(0x1d, (4, 4));
    map.insert(0x1e, (4, 4));
    map.insert(0x1f, (4, 4));
    map.insert(0x20, (4, 4));
    map.insert(0x2d, (4, 4));
    map.insert(0x2e, (5, 4));
    map.insert(0x2f, (5, 5));
    map.insert(0x30, (6, 5));
    map.insert(0x31, (6, 6));
    map.insert(0x32, (8, 5));
    map.insert(0x33, (8, 6));
    map.insert(0x34, (8, 8));
    map.insert(0x35, (10, 5));
    map.insert(0x36, (10, 6));
    map.insert(0x37, (10, 8));
    map.insert(0x38, (10, 10));
    map.insert(0x39, (12, 10));
    map.insert(0x3a, (12, 12));
    Mutex::new(map)
});

pub static BPPS: Lazy<Mutex<HashMap<u32, u32>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0x1, 1);
    map.insert(0x2, 1);
    map.insert(0x3, 2);
    map.insert(0x4, 2);
    map.insert(0x5, 2);
    map.insert(0x6, 2);
    map.insert(0x7, 2);
    map.insert(0x8, 2);
    map.insert(0x9, 2);
    map.insert(0xb, 4);
    map.insert(0xc, 4);
    map.insert(0xe, 4);
    map.insert(0x1a, 8);
    map.insert(0x1b, 0x10);
    map.insert(0x1c, 0x10);
    map.insert(0x1d, 8);
    map.insert(0x1e, 0x10);
    map.insert(0x1f, 0x10);
    map.insert(0x20, 0x10);
    map.insert(0x2d, 0x10);
    map.insert(0x2e, 0x10);
    map.insert(0x2f, 0x10);
    map.insert(0x30, 0x10);
    map.insert(0x31, 0x10);
    map.insert(0x32, 0x10);
    map.insert(0x33, 0x10);
    map.insert(0x34, 0x10);
    map.insert(0x35, 0x10);
    map.insert(0x36, 0x10);
    map.insert(0x37, 0x10);
    map.insert(0x38, 0x10);
    map.insert(0x39, 0x10);
    map.insert(0x3a, 0x10);
    map.insert(0x3b, 2);
    Mutex::new(map)
});

#[inline]
pub fn round_up(x: u32, y: u32) -> u32 {
    ((x - 1) | (y - 1)) + 1
}

#[inline]
pub fn div_round_up(n: u32, d: u32) -> u32 {
    (n + d - 1) / d
}

#[inline]
pub fn pow2_round_up(mut x: u32) -> u32 {
    x -= 1;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x + 1
}

pub fn get_addr_block_linear(
    mut x: u32,
    y: u32,
    width: u32,
    bpp: u32,
    base_addr: u32,
    block_height: u32,
) -> u32 {
    let image_width_in_gobs = div_round_up(width * bpp, 64);
    let gob_address = base_addr
        + (y / (8 * block_height)) * 512 * block_height * image_width_in_gobs
        + (x * bpp / 64) * 512 * block_height
        + (y % (8 * block_height) / 8) * 512;

    x *= bpp;

    gob_address
        + ((x % 64) / 32) * 256
        + ((y % 8) / 2) * 64
        + ((x % 32) / 16) * 32
        + (y % 2) * 16
        + (x % 16)
}

#[allow(unused)]
pub fn get_block_height(height: u32) -> u32 {
    let mut block_height = pow2_round_up(height / 8);
    if block_height > 16 {
        block_height = 16;
    }
    block_height
}

#[allow(clippy::too_many_arguments)]
pub fn deswizzle(
    width: u32,
    height: u32,
    blk_width: u32,
    blk_height: u32,
    round_pitch: bool,
    bpp: u32,
    tile_mode: u16,
    block_height_log2: u32,
    buffer: Vec<u8>,
) -> Result<Vec<u8>, Error> {
    if block_height_log2 > 5 {
        return Err(Error::CorruptData);
    }

    let block_height = 1 << block_height_log2;
    let width = div_round_up(width, blk_width);
    let height = div_round_up(height, blk_height);

    let (pitch, surf_size) = if tile_mode == 1 {
        let mut pitch = width * bpp;
        if round_pitch {
            pitch = round_up(pitch, 32)
        }
        let surf_size = pitch * height;
        (pitch, surf_size)
    } else {
        let pitch = round_up(width * bpp, 64);
        let surf_size = pitch * round_up(height, block_height * 8);
        (pitch, surf_size)
    };

    let mut res = vec![0; surf_size as usize];

    for y in 0..height {
        for x in 0..width {
            let pos = if tile_mode == 1 {
                y * pitch + x * bpp
            } else {
                get_addr_block_linear(x, y, width, bpp, 0, block_height)
            };

            let pos2 = (y * width + x) * bpp;

            if pos + bpp <= surf_size {
                res[pos2 as usize..(pos2 + bpp) as usize]
                    .copy_from_slice(&buffer[pos as usize..(pos + bpp) as usize]);
            }
        }
    }
    Ok(res)
}
