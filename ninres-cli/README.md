# Ninres-cli

A command-line tool to handle commonly used Nintendo file formats.

Please refer to the [Wiki](https://github.com/Kinnay/Nintendo-File-Formats/wiki).

## Installation

```bash
cargo install ninres-cli
```

## Usage

```bash
ninres [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    extract    Extract assets from given input file
    help       Prints this message or the help of the given subcommand(s)
```

```bash
ninres extract --input <input> --output <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>
    -o, --output <output>
```
