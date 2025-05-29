use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub claude_api_key: String,
    pub model: String,
    pub max_tokens: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            claude_api_key: String::new(),
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 300,
        }
    }
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        path.push("quick-question");
        Ok(path)
    }
    
    pub fn config_path() -> Result<PathBuf> {
        let mut path = Self::config_dir()?;
        path.push("config.toml");
        Ok(path)
    }
    
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        
        if !path.exists() {
            return Err(anyhow::anyhow!("Config file not found"));
        }
        
        let content = fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;
        
        // Check for environment variable override
        if let Ok(api_key) = std::env::var("CLAUDE_API_KEY") {
            config.claude_api_key = api_key;
        }
        
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;
        fs::create_dir_all(&config_dir)?;
        
        let path = Self::config_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        
        Ok(())
    }
}
