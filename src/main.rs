mod process;

use process::{ManaboxConfig, init_mana};
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "mn")]
#[command(about = "mana: A simple and intuitive version management tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the mana box
    Init {
        /// Specify the box name (default: current folder name)
        #[arg(short, long)]
        name: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { name }) => {
            init_mana(name)?;
        }
        None => {
    println!("Hello mana!");
    if let Ok(config) = ManaboxConfig::load() {
        let snapshot = process::scan_workspace(&config)?;
        println!("✅ Snapshot created with {} files.", snapshot.files.len());
        // Optionally: Print one for a quick check
        if let Some((path, hash)) = snapshot.files.iter().next() {
            println!("Example: {} -> {}", path, &hash[..8]); // Show first 8 chars of hash
        }
    }
}
    }

    Ok(())
}