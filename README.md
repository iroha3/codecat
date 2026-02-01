<div align="center">

<img src="docs/logo.png" alt="codecat logo" width="150"/>

# codecat

[![crates.io](https://img.shields.io/crates/v/codecat.svg)](https://crates.io/crates/codecat)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

**「 Merge Code Repository into a Single File | Respects `.gitignore` | Ideal for LLM Code Analysis 」**

[中文说明](README.zh.md)

</div>

`codecat` is a command-line utility that can merge a code repository into a single text file and split it back. It respects the rules defined in your `.gitignore`, making it suitable for creating a clean context bundle of your project's source code for LLM analysis.

## Features

- **Merge & Split**: Easily bundle your project or restore it from a bundle.
- **Ignore Rules**: Respects `.gitignore`, `.ignore`, global ignore rules, etc., powered by the `ignore` crate.
- **Clear Delimiters**: Adds clear markers (`--- START FILE: path/to/file ---` and `--- END FILE: path/to/file ---`) for LLM readability.
- **UTF-8 Support**: Attempts to read files as UTF-8, skipping non-UTF8 files with a warning.
- **Cross-platform**: Works on Windows, macOS, and Linux.

## Installation

Ensure you have Rust and Cargo installed.

```bash
cargo install codecat
```

## Usage

`codecat` automatically detects whether to merge or split based on the input path.

### 1. Merging a Directory

Navigate to your project directory or provide the path to it:

```bash
codecat /path/to/your/project
```

By default, `codecat` will create a file named `<project_directory_name>.codecat.txt` in the _parent_ directory of your project.

You can specify a custom output file path using the `-o` or `--output` flag:

```bash
codecat . -o context.txt
```

### 2. Splitting a Merged File

To restore a directory from a `.codecat.txt` file:

```bash
codecat my_project.codecat.txt
```

By default, `codecat` will create a directory with the same name as the file (excluding the `.codecat.txt` suffix) in the same location.

You can specify a custom output directory using the `-o` or `--output` flag:

```bash
codecat bundle.codecat.txt -o ./restored_project
```

## Example Output Structure

The generated file will look something like this:

```text
--- START FILE: src/main.rs ---
fn main() {
    println!("Hello, world!");
}

--- END FILE: src/main.rs ---

--- START FILE: Cargo.toml ---
[package]
name = "my_project"
version = "0.1.0"
# ... other cargo stuff ...

--- END FILE: Cargo.toml ---

--- START FILE: README.md ---
# My Project
This is my awesome project.
...
--- END FILE: README.md ---

```

_(Files listed in `.gitignore`, like `target/` or `.env`, will not be included)._

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
