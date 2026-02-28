use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManaboxConfig {
    pub file: Vec<String>,
    pub must: Vec<String>,
    pub select: Vec<String>,
}

impl ManaboxConfig {
    /// Loads the .manabox configuration file.
    pub fn load() -> Result<Self> {
        let content = fs::read_to_string(".manabox")?;
        let config: ManaboxConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

/// Initializes the mana box environment.
/// Creates .mana directory structure and a default .manabox file.
pub fn init_mana(name: &Option<String>) -> Result<()> {
    // 1. Determine the box name
    let box_name = name.clone().unwrap_or_else(|| {
        env::current_dir()
            .expect("Failed to get current directory")
            .file_name()
            .expect("Failed to get directory name")
            .to_string_lossy()
            .into_owned()
    });

    // 2. Create .mana directory (with existence check)
    if Path::new(".mana").exists() {
        println!("⚠️ mana: A box (.mana) already exists.");
    } else {
        fs::create_dir_all(".mana/objects")?;
        fs::create_dir_all(".mana/storage/main")?;
        fs::write(".mana/now", "main")?;
        fs::write(".mana/objects/main", "none")?;
        println!("✨ create box \"{}\".", box_name);
    }

    // 3. Create .manabox file (with overwrite protection)
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