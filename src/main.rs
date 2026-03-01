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
            // Check if .manabox exists and show debug info for development
            if let Ok(config) = ManaboxConfig::load() {
                println!("Config loaded successfully:");
                println!("  File:  {:?}", config.file);
                println!("  Must:  {:?}", config.must);
                println!("  Select:  {:?}", config.select);
            }
        }
    }

    Ok(())
}