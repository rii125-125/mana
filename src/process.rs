use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::path::Path;
use anyhow::Result;
use walkdir::WalkDir;
use std::collections::HashSet;

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

// Add this to your existing ManaboxConfig struct or as a standalone function
/// Scans the current directory and categorizes files based on .manabox rules.
pub fn scan_workspace(config: &ManaboxConfig) -> Result<()> {
    println!("🔍 Scanning workspace...");

    // Convert config lists to HashSets for faster lookup
    // Hint: Remove trailing slashes for easier comparison
    let ignore_set: HashSet<String> = config.file.iter().map(|s| s.trim_end_matches('/').to_string()).collect();
    let must_set: HashSet<_> = config.must.iter().collect();
    let select_set: HashSet<_> = config.select.iter().collect();

    let walker = WalkDir::new(".").into_iter();

    for entry in walker.filter_entry(|e| {
        // Efficiency: If the directory is in the ignore list, don't even enter it!
        let name = e.file_name().to_string_lossy();
        if name == ".mana" || ignore_set.contains(&name.to_string()) {
            return false; // Skip this directory entirely
        }
        true
    }) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            // Classification logic
            if must_set.contains(&file_name) {
                println!("  [Must  ] {:?}", path);
            } else if select_set.contains(&file_name) {
                println!("  [Select] {:?}", path);
            } else {
                // If it's not ignored (already filtered) and not Must/Select, it's Other
                println!("  [Other ] {:?}", path);
            }
        }
    }

    Ok(())
}