mod process;

use process::ManaboxConfig;

use clap::{Parser, Subcommand};
use std::fs;
use std::env;
use std::path::Path;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "mn")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the box
    Init {
        /// Specify the box name (default is the current folder name)
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
        }
    }

    // テスト呼び出し
    if let Ok(config) = ManaboxConfig::load() {
        println!("Config loaded from process.rs: {:?}", config);
    }
    Ok(())
}

fn init_mana(name: &Option<String>) -> Result<()> {
    // 1. ボックス名の決定
    let box_name = name.clone().unwrap_or_else(|| {
        env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    });
    // 2. .mana ディレクトリの作成（既存チェック付き）
    if Path::new(".mana").exists() {
        println!("⚠️ mana: A box (.mana) already exists.");
    } else {
        fs::create_dir_all(".mana/objects")?;
        fs::create_dir_all(".mana/storage/main")?;
        fs::write(".mana/now", "main")?;
        fs::write(".mana/objects/main", "none")?;
        println!("✨ create box \"{}\".", box_name);
    }

    // 3. .manabox の作成（上書き防止ガード）
    if Path::new(".manabox").exists() {
        println!("✋ mana: Since '.manabox' already exists, creation was skipped.");
    } else {
        let default_manabox = r#"file: [
    "node_modules/",
    "target/",
    "out/",
    ".vscode/",
    "dist/",
    "build/",
    "__pycache__/",
    ".env",
    ".DS_Store",
    "Thumbs.db",
    ".class",
    ".log",
]
must: [
    "package.json",
    "package-lock.json",
    "Cargo.toml",
    "Cargo.lock",
]
select: [
    "README.md",
]
"#;
        fs::write(".manabox", default_manabox)?;
        println!("📄 A new '.manabox' has been created.");
    }

    Ok(())
}