//! Reads SARC files. A commonly used Nintendo file format.
//!
//! See http://mk8.tockdom.com/wiki/SARC_(File_Format)

#[cfg(feature = "zstd")]
#[macro_use]
extern crate cfg_if;

mod error;

use error::*;
use num_enum::TryFromPrimitive;
use std::{convert::TryFrom, str};

#[cfg(feature = "tar_sarc")]
use std::io::Cursor;

pub type Error = SarcError;

#[derive(Debug)]
pub struct Sarc<'a> {
    header: SarcHeader,
    sfat_header: SfatHeader,
    sfat_nodes: Vec<SfatNode<'a>>,
}

#[derive(Debug)]
pub struct SarcHeader {
    byte_order: ByteOrder,
    file_size: u32,
    data_offset: u32,
    version_number: u16,
}

#[derive(Debug)]
pub struct SfatHeader {
    node_count: u16,
}

#[derive(Debug)]
pub struct SfatNode<'a> {
    hash: u32,
    attributes: u32,
    name_table_offset: Option<u32>,
    name: Option<String>,
    data_start_offset: u32,
    data_end_offset: u32,
    data: &'a [u8],
    #[cfg(feature = "zstd")]
    data_deflated: Option<Vec<u8>>,
}

impl<'a> SfatNode<'a> {
    fn get_hash(data: &[u32], length: usize, key: u32) -> u32 {
        let mut result = 0;
        for i in 0..length {
            result = data[i] + result * key;
        }
        return result;
    }
}

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum ByteOrder {
    BigEndian = 0xfeff,
    LittleEndian = 0xfffe,
}

impl<'a> Sarc<'a> {
    #[cfg(feature = "tar_sarc")]
    pub fn into_tar(self) -> Result<Cursor<Vec<u8>>, Error> {
        let res = vec![];
        let cursor = Cursor::new(res);
        let mut builder = tar::Builder::new(cursor);

        self.sfat_nodes
            .into_iter()
            .map(|node| -> Result<(), Error> {
                if let Some(name) = node.name {
                    let mut header = tar::Header::new_gnu();
                    header.set_size(node.data.len() as u64);
                    cfg_if! {
                        if #[cfg(feature = "zstd")] {
                            builder.append_data(&mut header, name.clone(), node.data)?;
                            if let Some(data_deflated) = node.data_deflated {
                                let mut header = tar::Header::new_gnu();
                                header.set_size(data_deflated.len() as u64);
                                builder.append_data(&mut header, format!("{}.tar", name), &data_deflated[..])?;
                            }
                        } else {
                            builder.append_data(&mut header, name, node.data)?;
                        }
                    }
                }
                Ok(())
            })
            .collect::<Result<(), Error>>()?;
        builder.finish()?;
        Ok(builder.into_inner()?)
    }
}

pub fn read_sarc(sarc_file: &[u8]) -> Result<Sarc, Error> {
    let byte_order = ByteOrder::try_from(read_u16(sarc_file, 0x6, ByteOrder::BigEndian))?;
    let file_size = read_u32(sarc_file, 0x8, byte_order);
    let data_offset = read_u32(sarc_file, 0xC, byte_order);
    let version_number = read_u16(sarc_file, 0x10, byte_order);
    let node_count = read_u16(sarc_file, 0x14 + 0x6, byte_order);
    let mut sfat_nodes = vec![];
    let file_name_table_offset = (0x14 + 0xC + node_count as u32 * 0x10) as usize;
    for i in 0..node_count {
        let offset = (0x14 + 0xC + i * 0x10) as usize;
        let hash = read_u32(sarc_file, offset, byte_order);
        let attributes = read_u32(sarc_file, offset + 0x4, byte_order);
        let name_table_offset = if attributes & 0xffff0000 == 0x01000000 {
            Some((attributes & 0x0000ffff) * 4)
        } else {
            None
        };
        let name = if let Some(name_table_offset) = name_table_offset {
            let name = sarc_file[(file_name_table_offset + name_table_offset as usize + 8)..]
                .iter()
                .take_while(|&n| n != &0u8)
                .cloned()
                .collect::<_>();
            Some(String::from_utf8(name)?)
        } else {
            None
        };
        let data_start_offset = read_u32(sarc_file, offset + 0x8, byte_order);
        let data_end_offset = read_u32(sarc_file, offset + 0xC, byte_order);
        let data = &sarc_file
            [(data_offset + data_start_offset) as usize..(data_offset + data_end_offset) as usize];
        sfat_nodes.push(SfatNode {
            hash,
            attributes,
            name_table_offset,
            name,
            data_start_offset,
            data_end_offset,
            data,
            #[cfg(feature = "ruzstd")]
            data_deflated: if b"\x28\xB5\x2F\xFD" == &data[..4] {
                use std::io::Read;
                let mut decompressed = vec![];
                let mut cursor = Cursor::new(data);
                let mut decoder =
                    ruzstd::StreamingDecoder::new(&mut cursor).map_err(|_| Error::ZstdError)?;

                decoder.read_to_end(&mut decompressed).unwrap();
                Some(decompressed)
            } else {
                None
            },
        })
    }
    Ok(Sarc {
        header: SarcHeader {
            byte_order,
            file_size,
            data_offset,
            version_number,
        },
        sfat_header: SfatHeader { node_count },
        sfat_nodes,
    })
}

fn read_u16(file: &[u8], offset: usize, byte_order: ByteOrder) -> u16 {
    let from_bytes = match byte_order {
        ByteOrder::BigEndian => u16::from_be_bytes,
        ByteOrder::LittleEndian => u16::from_le_bytes,
    };
    from_bytes([file[offset], file[offset + 1]])
}

fn read_u32(file: &[u8], offset: usize, byte_order: ByteOrder) -> u32 {
    let from_bytes = match byte_order {
        ByteOrder::BigEndian => u32::from_be_bytes,
        ByteOrder::LittleEndian => u32::from_le_bytes,
    };
    from_bytes([
        file[offset],
        file[offset + 1],
        file[offset + 2],
        file[offset + 3],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static M1_MODEL_PACK: &[u8] = include_bytes!("../assets/M1_Model.pack");
    static M3_MODEL_PACK: &[u8] = include_bytes!("../assets/M3_Model.pack");
    static MW_MODEL_PACK: &[u8] = include_bytes!("../assets/MW_Model.pack");

    #[test_case(M1_MODEL_PACK; "with M1 Model Pack")]
    #[test_case(M3_MODEL_PACK; "with M3 Model Pack")]
    #[test_case(MW_MODEL_PACK; "with MW Model Pack")]
    fn test_read_sarc(sarc_file: &[u8]) {
        let sarc_file = read_sarc(sarc_file);

        assert!(sarc_file.is_ok());
    }

    #[cfg(feature = "tar")]
    #[test_case(M1_MODEL_PACK, "M1_Model.tar"; "with M1 Model Pack")]
    #[test_case(M3_MODEL_PACK, "M3_Model.tar"; "with M3 Model Pack")]
    #[test_case(MW_MODEL_PACK, "MW_Model.tar"; "with MW Model Pack")]
    fn test_into_tar(sarc_file: &[u8], file_name: &str) -> Result<(), Error> {
        let sarc_file = read_sarc(sarc_file)?;
        let tar = sarc_file.into_tar()?;

        // use std::io::Write;
        // let mut file = std::fs::File::create(file_name)?;
        // file.write_all(&tar.into_inner()[..])?;
        Ok(())
    }
}
