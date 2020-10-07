//! Reads SARC files.
//!
//! See http://mk8.tockdom.com/wiki/SARC_(File_Format)

#[cfg(feature = "tar_ninres")]
use crate::IntoTar;
use crate::{read_u16, read_u32, ByteOrderMask, Error};

use std::convert::TryFrom;

#[cfg(any(feature = "tar_ninres", feature = "zstd"))]
use std::io::Cursor;

#[derive(Clone, Debug)]
pub struct Sarc {
    pub header: SarcHeader,
    pub sfat_header: SfatHeader,
    pub sfat_nodes: Vec<SfatNode>,
}

#[derive(Clone, Debug)]
pub struct SarcHeader {
    pub byte_order: ByteOrderMask,
    pub file_size: u32,
    pub data_offset: u32,
    pub version_number: u16,
}

#[derive(Clone, Debug)]
pub struct SfatHeader {
    pub node_count: u16,
}

#[derive(Clone, Debug)]
pub struct SfatNode {
    pub hash: u32,
    pub attributes: u32,
    pub path_table_offset: Option<u32>,
    pub path: Option<String>,
    pub data_start_offset: u32,
    pub data_end_offset: u32,
    pub data: Vec<u8>,
    #[cfg(feature = "zstd")]
    pub data_decompressed: Option<Vec<u8>>,
}

impl SfatNode {
    fn get_hash(data: &[u32], length: usize, key: u32) -> u32 {
        let mut result = 0;
        for i in 0..length {
            result = data[i] + result * key;
        }
        return result;
    }
}

impl Sarc {
    pub fn new(buffer: &[u8]) -> Result<Sarc, Error> {
        let byte_order = ByteOrderMask::try_from(read_u16(buffer, 0x6, ByteOrderMask::BigEndian))?;
        let file_size = read_u32(buffer, 0x8, byte_order);
        let data_offset = read_u32(buffer, 0xC, byte_order);
        let version_number = read_u16(buffer, 0x10, byte_order);
        let node_count = read_u16(buffer, 0x14 + 0x6, byte_order);
        let mut sfat_nodes = vec![];
        let file_name_table_offset = (0x14 + 0xC + node_count as u32 * 0x10) as usize;
        for i in 0..node_count {
            let offset = (0x14 + 0xC + i * 0x10) as usize;
            let hash = read_u32(buffer, offset, byte_order);
            let attributes = read_u32(buffer, offset + 0x4, byte_order);
            let name_table_offset = if attributes & 0xffff0000 == 0x01000000 {
                Some((attributes & 0x0000ffff) * 4)
            } else {
                None
            };
            let name = if let Some(name_table_offset) = name_table_offset {
                let name = buffer[(file_name_table_offset + name_table_offset as usize + 8)..]
                    .iter()
                    .take_while(|&n| n != &0u8)
                    .cloned()
                    .collect::<_>();
                Some(String::from_utf8(name)?)
            } else {
                None
            };
            let data_start_offset = read_u32(buffer, offset + 0x8, byte_order);
            let data_end_offset = read_u32(buffer, offset + 0xC, byte_order);
            let data = &buffer[(data_offset + data_start_offset) as usize
                ..(data_offset + data_end_offset) as usize];
            sfat_nodes.push(SfatNode {
                hash,
                attributes,
                path_table_offset: name_table_offset,
                path: name,
                data_start_offset,
                data_end_offset,
                data: data.to_vec(),
                #[cfg(feature = "zstd")]
                data_decompressed: if b"\x28\xB5\x2F\xFD" == &data[..4] {
                    use std::io::Read;
                    let mut decompressed = vec![];
                    let mut cursor = Cursor::new(data);
                    let mut decoder = ruzstd::StreamingDecoder::new(&mut cursor)
                        .map_err(|err| Error::ZstdError(err))?;

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
}

#[cfg(feature = "tar_ninres")]
impl IntoTar for Sarc {
    fn into_tar(self, mode: u32) -> Result<Cursor<Vec<u8>>, Error> {
        use std::time::SystemTime;

        let res = vec![];
        let cursor = Cursor::new(res);
        let mut builder = tar::Builder::new(cursor);
        let mtime = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.sfat_nodes
            .into_iter()
            .map(|node| -> Result<(), Error> {
                if let Some(name) = node.path {
                    let mut header = tar::Header::new_gnu();
                    header.set_size(node.data.len() as u64);
                    header.set_mode(mode);
                    header.set_mtime(mtime);
                    cfg_if! {
                        if #[cfg(feature = "zstd")] {
                            builder.append_data(&mut header, name.clone(), &node.data[..])?;
                            if let Some(data_deflated) = node.data_decompressed {
                                let mut header = tar::Header::new_gnu();
                                header.set_size(data_deflated.len() as u64);
                                header.set_cksum();
                                builder.append_data(&mut header, format!("{}.tar", name), &data_deflated[..])?;
                            }
                        } else {
                            header.set_cksum();
                            builder.append_data(&mut header, name, &node.data[..])?;
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
        let sarc_file = Sarc::new(sarc_file);

        assert!(sarc_file.is_ok());
    }

    #[cfg(feature = "tar")]
    #[test_case(M1_MODEL_PACK, "M1_Model.tar"; "with M1 Model Pack")]
    #[test_case(M3_MODEL_PACK, "M3_Model.tar"; "with M3 Model Pack")]
    #[test_case(MW_MODEL_PACK, "MW_Model.tar"; "with MW Model Pack")]
    fn test_into_tar<'a>(sarc_file: &'a [u8], file_name: &str) -> Result<(), Error> {
        let sarc_file = Sarc::new(sarc_file)?;
        let tar = sarc_file.into_tar(0o644)?;

        use std::io::Write;
        let mut file = std::fs::File::create(file_name)?;
        file.write_all(&tar.into_inner()[..])?;
        Ok(())
    }
}
