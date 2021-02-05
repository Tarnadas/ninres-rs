use color_eyre::eyre::Result;
use ninres::{NinRes, NinResFile, Sarc};
use std::{
    fs::{self, read},
    path::PathBuf,
};
use structopt::StructOpt;

/// A command-line tool to handle commonly used Nintendo files formats.
#[derive(StructOpt, Debug)]
#[structopt(name = "ninres")]
struct Opt {
    #[structopt(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum Cmd {
    /// Extract assets from given input file
    Extract(ExtractOpt),
}

#[derive(StructOpt, Debug, PartialEq)]
pub struct ExtractOpt {
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    pub output: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::from_args();

    match opt.cmd {
        Some(Cmd::Extract(extract_options)) => {
            let buffer = read(extract_options.input)?;
            let ninres = buffer.as_ninres()?;

            match &ninres {
                NinResFile::Bfres(_bfres) => {}
                NinResFile::Sarc(sarc) => {
                    extract_sarc(sarc, extract_options.output)?;
                }
            }
        }
        None => {
            Opt::clap().print_help()?;
        }
    }

    Ok(())
}

fn extract_sarc(sarc: &Sarc, out_path: PathBuf) -> Result<()> {
    sarc.sfat_nodes
        .iter()
        .map(move |sfat| -> Result<_> {
            let mut path = out_path.clone();
            if let Some(sfat_path) = &sfat.path {
                path.push(sfat_path);
                let mut folder_path = path.clone();
                folder_path.pop();
                if !folder_path.exists() {
                    fs::create_dir_all(folder_path)?;
                }

                let data = if let Some(data) = &sfat.data_decompressed {
                    data
                } else {
                    &sfat.data
                };

                if let Ok(file) = data.as_ninres() {
                    path.set_extension(file.get_extension().to_string());
                    if let NinResFile::Sarc(sarc) = &file {
                        let mut base_path = path.clone();
                        base_path.pop();
                        base_path.push(path.file_stem().unwrap());
                        extract_sarc(sarc, base_path)?;
                    }
                }
                fs::write(path, data)?;
            }
            Ok(())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(())
}
