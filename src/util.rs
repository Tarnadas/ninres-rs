use crate::ByteOrderMask;

pub fn read_u16(file: &[u8], offset: usize, bom: ByteOrderMask) -> u16 {
    let from_bytes = match bom {
        ByteOrderMask::BigEndian => u16::from_be_bytes,
        ByteOrderMask::LittleEndian => u16::from_le_bytes,
    };
    from_bytes([file[offset], file[offset + 1]])
}

pub fn read_u32(file: &[u8], offset: usize, bom: ByteOrderMask) -> u32 {
    let from_bytes = match bom {
        ByteOrderMask::BigEndian => u32::from_be_bytes,
        ByteOrderMask::LittleEndian => u32::from_le_bytes,
    };
    from_bytes([
        file[offset],
        file[offset + 1],
        file[offset + 2],
        file[offset + 3],
    ])
}

pub fn read_i32(file: &[u8], offset: usize, bom: ByteOrderMask) -> i32 {
    let from_bytes = match bom {
        ByteOrderMask::BigEndian => i32::from_be_bytes,
        ByteOrderMask::LittleEndian => i32::from_le_bytes,
    };
    from_bytes([
        file[offset],
        file[offset + 1],
        file[offset + 2],
        file[offset + 3],
    ])
}
