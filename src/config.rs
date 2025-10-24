use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "api-key")]
    pub api_key: String,
    #[serde(rename = "hub-url")]
    pub hub_url: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .context("Failed to read config file")?;
        
        toml::from_str(&content)
            .context("Failed to parse config file")
    }
}
