use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::path::Path;
use anyhow::Result;
use walkdir::WalkDir;
use std::collections::HashSet;
use sha2::{Sha256, Digest};
use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManaboxConfig {
    pub file: Vec<String>,
    pub must: Vec<String>,
    pub select: Vec<String>,
}

/// Represents a snapshot of the workspace files and their hashes.
#[derive(Debug)]
pub struct FileSnapshot {
    pub files: HashMap<String, String>, // Key: Relative Path, Value: Hash
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
/// Scans the workspace and returns a FileSnapshot containing hashes of all relevant files.
pub fn scan_workspace(config: &ManaboxConfig) -> Result<FileSnapshot> {
    println!("🔍 Scanning and hashing files...");
    let mut files = HashMap::new();

    let ignore_set: HashSet<String> = config.file.iter().map(|s| s.trim_end_matches('/').to_string()).collect();

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name != ".mana" && !ignore_set.contains(&name.to_string())
        })
        .filter_map(|e| e.ok()) 
    {
        let path = entry.path();
        if path.is_file() {
            // Calculate the hash for each file found
            let hash = calculate_hash(path)?;
            let path_str = path.to_string_lossy().to_string();
            files.insert(path_str, hash);
        }
    }

    Ok(FileSnapshot { files })
}

/// Calculates the SHA-256 hash of a file.
/// This is the "fingerprint" of the file content.
pub fn calculate_hash(path: &std::path::Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024]; // Read in chunks for efficiency

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
    }

    Ok(hex::encode(hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_calculate_hash() -> anyhow::Result<()> {
        // 1. 一時ディレクトリを作成
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("test.txt");

        // 2. fs::write を使って、書き込み直後にファイルを「完全に閉じる」
        // これでOSのバッファもフラッシュされ、ロックも解除される
        fs::write(&file_path, "hello mana")?;

        // 3. ハッシュ計算（ここから新しく File::open するので必ず成功する）
        let hash = calculate_hash(&file_path)?;

        // SHA-256 of "hello mana"
        let expected = "274a7732296c09819970921a8d0034606f2e8f19293114d2e057388716399676";
        
        assert_eq!(hash, expected);
        Ok(())
    }
}