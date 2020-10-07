use anyhow::Result;
use ninres::{NinRes, NinResFile, Sarc};
use std::{fs, path::PathBuf};

static M1_MODEL_PACK: &[u8] = include_bytes!("../assets/M1_Model.pack");

fn main() -> Result<()> {
    let sarc = Sarc::new(M1_MODEL_PACK);
    extract_sarc(sarc.unwrap(), "assets/extracted".into())
}

fn extract_sarc(sarc: Sarc, path: PathBuf) -> Result<()> {
    sarc.sfat_nodes
        .into_iter()
        .map(move |sfat| -> Result<_> {
            let mut path = path.clone();
            if let Some(sfat_path) = sfat.path {
                path.push(sfat_path);
                let mut folder_path = path.clone();
                folder_path.pop();
                if !folder_path.exists() {
                    fs::create_dir_all(folder_path)?;
                }

                let data = if let Some(data) = sfat.data_decompressed {
                    data
                } else {
                    sfat.data
                };

                match data.as_ninres() {
                    Ok(file) => {
                        path.set_extension(file.get_extension().to_string());
                        if let NinResFile::Sarc(sarc) = file {
                            let mut base_path = path.clone();
                            base_path.pop();
                            base_path.push(path.file_stem().unwrap());
                            extract_sarc(sarc, base_path)?;
                        }
                    }
                    _ => {}
                }
                fs::write(path, data)?;
            }
            Ok(())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(())
}
