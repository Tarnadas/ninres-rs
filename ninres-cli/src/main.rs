use color_eyre::eyre::Result;
use image::{DynamicImage, ImageBuffer};
use ninres::{Bfres, EmbeddedFile, NinRes, NinResFile, Sarc};
use std::{
    cmp,
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
                NinResFile::Bfres(bfres) => {
                    extract_bfres(bfres, extract_options.output)?;
                }
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

fn extract_bfres(bfres: &Bfres, out_path: PathBuf) -> Result<()> {
    for file in bfres.embedded_files.iter() {
        match file {
            EmbeddedFile::BNTX(bntx) => {
                for texture in bntx.textures.iter() {
                    for (tex_count, mips) in texture.texture_data.iter().enumerate() {
                        for (mip_level, mip) in mips.iter().enumerate() {
                            let width = cmp::max(1, texture.width >> mip_level);
                            let height = cmp::max(1, texture.height >> mip_level);
                            let buf = if let Some(image) =
                                ImageBuffer::from_raw(width, height, mip.clone())
                            {
                                image
                            } else {
                                // TODO ?
                                continue;
                            };
                            let image = DynamicImage::ImageRgba8(buf);

                            let mut path = out_path.clone();
                            if !path.exists() {
                                fs::create_dir(path.clone())?;
                            }
                            path.push(&format!("{}_{}_{}.png", texture.name, tex_count, mip_level));
                            if let Err(_err) = image.save(&path) {
                                // TODO
                            }
                        }
                    }
                }
            }
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
                    match &file {
                        NinResFile::Bfres(bfres) => {
                            let mut base_path = path.clone();
                            base_path.pop();
                            base_path.push(path.file_stem().unwrap());
                            extract_bfres(bfres, base_path)?;
                        }
                        NinResFile::Sarc(sarc) => {
                            let mut base_path = path.clone();
                            base_path.pop();
                            base_path.push(path.file_stem().unwrap());
                            extract_sarc(sarc, base_path)?;
                        }
                    }
                }
                fs::write(path, data)?;
            }
            Ok(())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(())
}
