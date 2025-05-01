use anyhow::{anyhow, Context, Result};
use clap::Parser;
use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "INPUT_DIR")]
    input: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let canonical_input_path = cli.input.canonicalize().with_context(|| {
        format!(
            "Failed to get absolute path for input directory: {}",
            cli.input.display()
        )
    })?;

    let output_path = match cli.output {
        Some(path) => path,
        None => {
            let dir_name = canonical_input_path
                .file_name()
                .ok_or_else(|| {
                    anyhow!(
                        "Could not determine directory name from canonical path: {}",
                        canonical_input_path.display()
                    )
                })?
                .to_string_lossy();

            let parent_dir = canonical_input_path.parent().ok_or_else(|| {
                anyhow!(
                    "Could not get parent directory for canonical path: {}",
                    canonical_input_path.display()
                )
            })?;

            let output_filename = format!("{}.codecat.txt", dir_name);
            let final_output_path = parent_dir.join(output_filename);

            if final_output_path.as_os_str().len() > 240 {
                PathBuf::from(format!(r#"\\?\{}"#, final_output_path.display()))
            } else {
                final_output_path
            }
        }
    };

    // --- Input validation (use original input for user message, canonical for checks) ---
    if !canonical_input_path.is_dir() {
        // Check the canonical path
        anyhow::bail!(
            "Input path '{}' (canonical: '{}') is not a valid directory.",
            cli.input.display(), // Show original path in error
            canonical_input_path.display()
        );
    }

    // Create BufWriter
    let output_file = File::create(&output_path)
        .with_context(|| format!("Failed to create output file '{}'", output_path.display()))?;
    let mut writer = BufWriter::new(output_file);

    println!("Processing directory: {}", cli.input.display()); // Show original path
    println!(
        "Processing canonical directory: {}",
        canonical_input_path.display() // Show canonical path
    );
    println!("Output will be written to: {}", output_path.display());

    // --- Use the canonical path for WalkBuilder ---
    let walker = WalkBuilder::new(&canonical_input_path).build();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path(); // This path is based on canonical_input_path
                if path.is_file() {
                    // --- Use the SAME canonical path for strip_prefix ---
                    if let Ok(relative_path) = path.strip_prefix(&canonical_input_path) {
                        // Use canonical_input_path here!
                        let relative_path_str = relative_path.to_string_lossy();
                        println!("Adding file: {}", relative_path_str); // This should now work

                        // Rest of your file writing logic...
                        writeln!(writer, "--- START FILE: {} ---", relative_path_str)
                            .with_context(|| {
                                format!("Failed to write start metadata for {}", relative_path_str)
                            })?;

                        match fs::read_to_string(path) {
                            Ok(content) => {
                                writer.write_all(content.as_bytes()).with_context(|| {
                                    format!("Failed to write content of {}", relative_path_str)
                                })?;
                                if !content.ends_with('\n') {
                                    writeln!(writer)?;
                                }
                            }
                            Err(e) => {
                                if e.kind() == io::ErrorKind::InvalidData {
                                    eprintln!(
                                        "Warning: Skipping non-UTF8 file: {}",
                                        relative_path_str
                                    );
                                    writeln!(writer, "[Skipped non-UTF8 file]")?;
                                } else {
                                    eprintln!(
                                        "Warning: Failed to read file {}: {}. Skipping content.",
                                        relative_path_str, e
                                    );
                                    writeln!(writer, "[Skipped due to read error: {}]", e)?;
                                }
                            }
                        }

                        writeln!(writer, "--- END FILE: {} ---", relative_path_str).with_context(
                            || format!("Failed to write end metadata for {}", relative_path_str),
                        )?;
                        writeln!(writer)?;
                    } else {
                        // This warning should be much less likely now
                        eprintln!(
                            "Warning: Could not determine relative path for {} using base {}",
                            path.display(),
                            canonical_input_path.display() // Log the base path used
                        );
                    }
                }
            }
            Err(err) => eprintln!("Warning: Error accessing entry: {}", err),
        }
    }

    writer.flush().context("Failed to flush output writer")?;

    println!(
        "Successfully processed repository and wrote to {}",
        output_path.display()
    );
    Ok(())
}
