use crate::{ByteOrderMark, Error};

#[derive(Clone, Debug)]
pub struct BNTX {
    pub header: BNTXHeader,
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

        // Alignment = loader.ReadByte();
        // TargetAddressSize = loader.ReadByte(); //Thanks MasterF0X for pointing out the layout of the these
        // uint OffsetToFileName = loader.ReadUInt32();
        // Flag = loader.ReadUInt16();
        // BlockOffset = loader.ReadUInt16();
        // uint RelocationTableOffset = loader.ReadUInt32();
        // uint sizFile = loader.ReadUInt32();
        // Target = loader.ReadChars(4);
        // int textureCount = loader.ReadInt32();
        // long TextureArrayOffset = loader.ReadInt64();

        let header = BNTXHeader {
            alignment,
            target_address_size,
            file_name_offset,
            flag,
            block_offset,
            relocation_table_offset,
            file_size,
        };
        Ok(Self { header })
    }
}
