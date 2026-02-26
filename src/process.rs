// src/process.rs
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManaboxConfig {
    pub file: Vec<String>,
    pub must: Vec<String>,
    pub select: Vec<String>,
}

impl ManaboxConfig {
    pub fn load() -> Result<Self> {
        let content = fs::read_to_string(".manabox")?;
        let config: ManaboxConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}