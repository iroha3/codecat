<div align="center">

<img src="docs/logo.png" alt="codecat logo" width="150"/>

# codecat

[![crates.io](https://img.shields.io/crates/v/codecat.svg)](https://crates.io/crates/codecat)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

**「 Merge Code Repository into a Single File | Respects `.gitignore` | Ideal for LLM Code Analysis 」**

[中文说明](README.zh.md)

</div>

`codecat` is a command-line utility that merges code from a repository into a single text file. It respects the rules defined in your `.gitignore`, making it suitable for creating a clean context bundle of your project's source code.

## Features

- Recursively walks through directories.
- Respects `.gitignore`, `.ignore`, global ignore rules, etc., powered by the `ignore` crate.
- Concatenates file contents into a single output file.
- Adds clear delimiters (`--- START FILE: path/to/file ---` and `--- END FILE: path/to/file ---`) for each file.
- Attempts to read files as UTF-8, skipping non-UTF8 files with a warning.
- Simple command-line interface.
- Cross-platform (Windows, macOS, Linux).

## Installation

Ensure you have Rust and Cargo installed.

```bash
cargo install codecat
```

## Usage

### Basic Usage

Navigate to your project directory or provide the path to it:

```bash
codecat /path/to/your/project
```

Or, if you are already in the project directory:

```bash
codecat .
```

By default, `codecat` will create a file named `<project_directory_name>.codecat.txt` in the _parent_ directory of your project.

### Specifying Output File

You can specify a custom output file path using the `-o` or `--output` flag:

```bash
codecat /path/to/your/project -o /path/to/output/bundle.txt
```

```bash
codecat . --output context.txt
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
