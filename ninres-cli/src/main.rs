use color_eyre::eyre::Result;
use ninres::{NinRes, NinResFile, Sarc};
use std::{fs, path::PathBuf};
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "ninres")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum Cmd {
    /// Extract assets from given input file
    Extract(ExtractOpt),
}

#[derive(StructOpt, Debug, PartialEq)]
pub struct ExtractOpt {
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::from_args();
    dbg!(&opt);

    Ok(())
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
