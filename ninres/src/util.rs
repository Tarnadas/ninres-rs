use crate::ByteOrderMark;

pub fn read_u16(file: &[u8], offset: usize, bom: ByteOrderMark) -> u16 {
    let from_bytes = match bom {
        ByteOrderMark::BigEndian => u16::from_be_bytes,
        ByteOrderMark::LittleEndian => u16::from_le_bytes,
    };
    from_bytes([file[offset], file[offset + 1]])
}

pub fn read_u32(file: &[u8], offset: usize, bom: ByteOrderMark) -> u32 {
    let from_bytes = match bom {
        ByteOrderMark::BigEndian => u32::from_be_bytes,
        ByteOrderMark::LittleEndian => u32::from_le_bytes,
    };
    from_bytes([
        file[offset],
        file[offset + 1],
        file[offset + 2],
        file[offset + 3],
    ])
}

pub fn read_i32(file: &[u8], offset: usize, bom: ByteOrderMark) -> i32 {
    let from_bytes = match bom {
        ByteOrderMark::BigEndian => i32::from_be_bytes,
        ByteOrderMark::LittleEndian => i32::from_le_bytes,
    };
    from_bytes([
        file[offset],
        file[offset + 1],
        file[offset + 2],
        file[offset + 3],
    ])
}
