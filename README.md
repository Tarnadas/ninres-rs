# Ninres-rs

![Continuous integration](https://github.com/Tarnadas/ninres-rs/workflows/Continuous%20integration/badge.svg)
[<img alt="blog.rust-lang.org" src="https://img.shields.io/badge/Rust-1.53-blue?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html)
[![Discord](https://img.shields.io/discord/168893527357521920?label=Discord&logo=discord&color=7289da)](https://discord.gg/SPZsgSe)

Read commonly used Nintendo file formats.

Please refer to the Wiki:
https://github.com/Kinnay/Nintendo-File-Formats/wiki

All file formats are behind feature flags.
Here is a list of available Nintendo file format features:

`bfres`, `sarc`

You can also enable additional features:

`tar`: write Nintendo resource to tar ball.

`zstd`: ZSTD decompression.

`png`: allows extracting textures as png.

The library is written in Rust and compiles to WebAssembly for the web or can be used as a standard Rust Crate.
A live demo running in your browser can be found here:
https://tarnadas.github.io/ninres-rs/

### Examples

Enable desired features in `Cargo.toml`.

```toml
[dependencies]
ninres = { version = "*", features = ["bfres", "sarc", "zstd"] }
```

In your `main.rs`.

```rust
use std::fs::read;
use ninres::{NinRes, NinResFile};

let buffer = read("foo.pack")?;
let ninres = buffer.as_ninres()?;

match &ninres {
    NinResFile::Bfres(_bfres) => {}
    NinResFile::Sarc(_sarc) => {}
}
```

## Write to tar

Convert resource into tar buffer.
This buffer can then e.g. be stored in a file.

The `mode` parameter refers to the file mode within the tar ball.

### Examples

```rust
use ninres::{sarc::Sarc, IntoTar};
use std::{fs::{read, File}, io::Write};

let sarc_file = Sarc::new(&read("./assets/M1_Model.pack")?)?;
let tar = sarc_file.into_tar(0o644)?;

let mut file = File::create("M1_Model.tar")?;
file.write_all(&tar.into_inner()[..])?;
```
