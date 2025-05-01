<div align="center">

<img src="docs/logo.png" alt="codecat logo" width="150"/>

# codecat

[![crates.io](https://img.shields.io/crates/v/codecat.svg)](https://crates.io/crates/codecat)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

**「代码仓库合并为单一文件 | 基于 `.gitignore` | 适合 LLM 代码分析」**

[English Readme](README.md)

</div>

`codecat` 是一个命令行工具，它可以将代码仓库的代码合并到一个单独的文本文件中。它会遵循你的 `.gitignore` 中定义的规则，适合为你的项目源代码创建一个干净的上下文包。

## 功能特性

- 递归遍历目录。
- 遵循 `.gitignore`、`.ignore`、全局忽略规则等，由强大的 `ignore` crate 驱动。
- 将文件内容合并到单个输出文件中。
- 为每个文件添加清晰的分隔符（`--- START FILE: path/to/file ---` 和 `--- END FILE: path/to/file ---`）。
- 尝试将文件读取为 UTF-8 编码，对于非 UTF-8 文件会跳过并显示警告。
- 简单的命令行接口。
- 跨平台（Windows, macOS, Linux）。

## 安装

请确保你已经安装了 Rust 和 Cargo。

```bash
cargo install codecat
```

## 使用方法

### 基本用法

导航到你的项目目录或提供项目路径：

```bash
codecat /path/to/your/project
```

或者，如果你已经在项目目录中：

```bash
codecat .
```

默认情况下，`codecat` 会在你的项目的*父*目录下创建一个名为 `<项目目录名>.codecat.txt` 的文件。

### 指定输出文件

你可以使用 `-o` 或 `--output` 标志指定自定义的输出文件路径：

```bash
codecat /path/to/your/project -o /path/to/output/bundle.txt
```

```bash
codecat . --output context.txt
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
