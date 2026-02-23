use clap::{Parser, Subcommand};
use std::fs;
use std::env;
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
            // 名前が決まっていない場合は現在のディレクトリ名を取得
            let box_name = name.clone().unwrap_or_else(|| {
                env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            });

            // フォルダ作成
            fs::create_dir_all(".mana/objects")?;
            fs::create_dir_all(".mana/storage/main")?; // storageの中にmainフォルダ
            fs::write(".mana/now", "main")?;
            fs::write(".mana/objects/main", "none")?;

            // .manabox の生成（先ほどのYAML案を書き込む）
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

            println!("create box \"{}\".", box_name);
        }
        None => {
            println!("Hello mana!");
        }
    }

    Ok(())
}