use anyhow::{Context, Result, anyhow};
use clap::Parser;
use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

const MERGE_FILE_SUFFIX: &str = ".codecat.txt";
const START_MARKER_PREFIX: &str = "--- START FILE: ";
const END_MARKER_SUFFIX: &str = " ---";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "INPUT_PATH")]
    input: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE_OR_DIR")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input_path = &cli.input;

    if !input_path.exists() {
        anyhow::bail!("Input path '{}' does not exist.", input_path.display());
    }

    if input_path.is_dir() {
        // 分支 1: 合并目录
        println!("Input is a directory. Running merge operation...");
        merge_directory(input_path, &cli.output)?;
    } else if input_path.is_file() && input_path.to_string_lossy().ends_with(MERGE_FILE_SUFFIX) {
        // 分支 2: 拆分文件
        println!("Input is a merge file. Running split operation...");
        split_file(input_path, &cli.output)?;
    } else {
        // 无效输入
        anyhow::bail!(
            "Input path '{}' is neither a directory nor a valid '{}' file.",
            input_path.display(),
            MERGE_FILE_SUFFIX
        );
    }

    Ok(())
}

/// 将目录内容合并为单个文本文件
fn merge_directory(input_dir: &Path, output_file: &Option<PathBuf>) -> Result<()> {
    let canonical_input_path = input_dir.canonicalize().with_context(|| {
        format!(
            "Failed to get absolute path for input directory: {}",
            input_dir.display()
        )
    })?;

    // --- 输出路径处理逻辑 (与原版相同) ---
    let output_path = match output_file {
        Some(path) => path.clone(),
        None => {
            let dir_name = canonical_input_path
                .file_name()
                .ok_or_else(|| {
                    anyhow!(
                        "Could not determine directory name from path: {}",
                        canonical_input_path.display()
                    )
                })?
                .to_string_lossy();
            let parent_dir = canonical_input_path.parent().ok_or_else(|| {
                anyhow!(
                    "Could not get parent directory for path: {}",
                    canonical_input_path.display()
                )
            })?;
            let output_filename = format!("{}{}", dir_name, MERGE_FILE_SUFFIX);
            parent_dir.join(output_filename)
        }
    };

    let file = File::create(&output_path)
        .with_context(|| format!("Failed to create output file '{}'", output_path.display()))?;
    let mut writer = BufWriter::new(file);

    println!("Processing directory: {}", canonical_input_path.display());
    println!("Output will be written to: {}", output_path.display());

    // --- 文件遍历与写入 (与原版相同) ---
    let walker = WalkBuilder::new(&canonical_input_path).build();

    for result in walker {
        let entry = result?;
        let path = entry.path();
        if path.is_file()
            && let Ok(relative_path) = path.strip_prefix(&canonical_input_path)
        {
            let relative_path_str = relative_path.to_string_lossy();
            println!("Adding file: {}", relative_path_str);

            writeln!(
                writer,
                "{}{}{}",
                START_MARKER_PREFIX, relative_path_str, END_MARKER_SUFFIX
            )?;

            match fs::read_to_string(path) {
                Ok(content) => {
                    writer.write_all(content.as_bytes())?;
                    if !content.ends_with('\n') {
                        writeln!(writer)?;
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::InvalidData => {
                    eprintln!("Warning: Skipping non-UTF8 file: {}", relative_path_str);
                    writeln!(writer, "[Skipped non-UTF8 file]")?;
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to read file {}: {}. Skipping.",
                        relative_path_str, e
                    );
                    writeln!(writer, "[Skipped due to read error: {}]", e)?;
                }
            }
            writeln!(
                writer,
                "--- END FILE: {}{}",
                relative_path_str, END_MARKER_SUFFIX
            )?;
            writeln!(writer)?;
        }
    }

    writer.flush().context("Failed to flush output writer")?;
    println!("Successfully merged directory to {}", output_path.display());
    Ok(())
}

/// 将合并后的文件拆分为原始目录结构
fn split_file(input_file: &Path, output_dir: &Option<PathBuf>) -> Result<()> {
    // --- 确定输出目录 ---
    let output_root = match output_dir {
        Some(path) => path.clone(),
        None => {
            let file_stem = input_file.file_stem().ok_or_else(|| {
                anyhow!("Could not get file stem from '{}'", input_file.display())
            })?;
            let parent_dir = input_file.parent().unwrap_or_else(|| Path::new("."));
            parent_dir.join(file_stem)
        }
    };

    if output_root.exists() && !output_root.is_dir() {
        anyhow::bail!(
            "Output path '{}' exists and is not a directory.",
            output_root.display()
        );
    }
    fs::create_dir_all(&output_root).with_context(|| {
        format!(
            "Failed to create output directory '{}'",
            output_root.display()
        )
    })?;

    println!(
        "Output will be written to directory: {}",
        output_root.display()
    );

    // --- 文件读取与解析 ---
    let file = File::open(input_file)
        .with_context(|| format!("Failed to open input file '{}'", input_file.display()))?;
    let reader = BufReader::new(file);

    let mut current_file_writer: Option<BufWriter<File>> = None;
    let mut file_count = 0;

    for line in reader.lines() {
        let line = line?;

        if let Some(path_str) = line
            .strip_prefix(START_MARKER_PREFIX)
            .and_then(|s| s.strip_suffix(END_MARKER_SUFFIX))
        {
            // 遇到新的 START 标记
            let relative_path = PathBuf::from(path_str);
            let absolute_path = output_root.join(&relative_path);

            if let Some(parent) = absolute_path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!(
                        "Failed to create parent directory for '{}'",
                        absolute_path.display()
                    )
                })?;
            }

            let new_file = File::create(&absolute_path)
                .with_context(|| format!("Failed to create file '{}'", absolute_path.display()))?;

            println!("Creating file: {}", relative_path.display());
            current_file_writer = Some(BufWriter::new(new_file));
            file_count += 1;
        } else if line.starts_with("--- END FILE:") {
            // 遇到 END 标记，关闭当前文件写入器
            if let Some(mut writer) = current_file_writer.take() {
                writer.flush()?;
            }
        } else if let Some(writer) = &mut current_file_writer {
            // 在 START 和 END 之间，写入文件内容
            writeln!(writer, "{}", line)?;
        }
    }

    println!(
        "Successfully split file into {} files in {}",
        file_count,
        output_root.display()
    );
    Ok(())
}
