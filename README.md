# Ninres-rs

![Continuous integration](https://github.com/Tarnadas/ninres-rs/workflows/Continuous%20integration/badge.svg)
[<img alt="blog.rust-lang.org" src="https://img.shields.io/badge/Rust-1.43-blue?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://blog.rust-lang.org/2020/04/23/Rust-1.43.0.html)

Read commonly used Nintendo file formats.

Please refer to the Wiki:
https://github.com/Kinnay/Nintendo-File-Formats/wiki

All file formats are behind feature flags.
Here is a list of available Nintendo file format features:

`bfres`, `sarc`

You can also enable additional features:

`tar_ninres`: write Nintendo resource to tar ball.

`zstd`: ZSTD decompression.

All features of this crate can be compiled to WebAssembly.

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
