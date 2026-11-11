use std::path::PathBuf;

use clap::{Parser, Subcommand};
use pt_core::*;

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

fn main() -> CoreResult {
    let cli = Cli::parse();
    let mut svs = ProjectService::default();
    match cli.command {
        Command::Extract { file } => {
            svs.add_pdf(&file)?;
            svs.save("./demo_project/project.json")?;
        }
    };
    Ok(())
}
