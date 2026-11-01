use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Extract text from a file
    Extract {
        /// Path to the file to extract from
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Extract { file } => match pt_core::extract(&file) {
            Ok(text) => println!("{text}"),
            Err(err) => {
                eprintln!("error: failed to extract {}: {err}", file.display());
                std::process::exit(1);
            }
        },
    }
}
