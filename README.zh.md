<div align="center">

<img src="docs/logo.png" alt="codecat logo" width="150"/>

# codecat

[![crates.io](https://img.shields.io/crates/v/codecat.svg)](https://crates.io/crates/codecat)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

**「代码仓库合并为单一文件 | 基于 `.gitignore` | 适合 LLM 代码分析」**

[English Readme](README.md)

</div>

`codecat` 是一个命令行工具，它可以将代码仓库合并到一个单独的文本文件中，也可以将其拆分还原。它会遵循你的 `.gitignore` 中定义的规则，非常适合为 LLM 代码分析提供干净的项目上下文。

## 功能特性

- **合并与拆分**：轻松打包项目或从打包文件中还原。
- **遵循忽略规则**：支持 `.gitignore`、`.ignore`、全局忽略规则等，由强大的 `ignore` crate 驱动。
- **清晰的分隔符**：为每个文件添加清晰的标记（`--- START FILE: path/to/file ---` 和 `--- END FILE: path/to/file ---`），便于 LLM 阅读。
- **UTF-8 支持**：尝试将文件读取为 UTF-8 编码，对于非 UTF-8 文件会跳过并显示警告。
- **跨平台**：支持 Windows, macOS, Linux。

## 安装

请确保你已经安装了 Rust 和 Cargo。

```bash
cargo install codecat
```

## 使用方法

`codecat` 会根据输入路径自动检测是执行合并还是拆分操作。

### 1. 合并目录

导航到你的项目目录或提供项目路径：

```bash
codecat /path/to/your/project
```

默认情况下，`codecat` 会在你的项目的*父*目录下创建一个名为 `<项目目录名>.codecat.txt` 的文件。

你可以使用 `-o` 或 `--output` 标志指定自定义的输出文件路径：

```bash
codecat . -o context.txt
```

### 2. 拆分合并文件

从 `.codecat.txt` 文件还原目录结构：

```bash
codecat my_project.codecat.txt
```

默认情况下，`codecat` 会在当前位置创建一个与文件名同名（去掉 `.codecat.txt` 后缀）的目录。

你可以使用 `-o` 或 `--output` 标志指定自定义的输出目录：

```bash
codecat bundle.codecat.txt -o ./restored_project
```

## 输出示例结构

生成的文件大致如下所示：

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
# ... 其他 cargo 配置 ...

--- END FILE: Cargo.toml ---

--- START FILE: README.md ---
# 我的项目
这是我的超赞项目。
...
--- END FILE: README.md ---

```

_(`.gitignore` 中列出的文件，如 `target/` 或 `.env`，将不会被包含在内)。_

## 许可证

本项目采用双重许可证，你可以任选其一：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT)
