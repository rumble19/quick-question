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
    
    pub fn custom_prompt_path() -> Result<PathBuf> {
        let mut path = Self::config_dir()?;
        path.push("custom_prompt.txt");
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
    
    pub fn create_custom_prompt_file(&self) -> Result<()> {
        let custom_prompt_path = Self::custom_prompt_path()?;
        
        // Only create if it doesn't exist
        if !custom_prompt_path.exists() {
            let default_content = "# Your custom prompt goes here\n# \n# This will be APPENDED to the default system prompt, so you can add\n# additional instructions without losing the original behavior.\n# \n# Examples:\n# - Always respond in a specific language\n# - Add domain-specific knowledge\n# - Modify the response style\n# - Add personality traits\n# \n# Delete these comments and add your custom instructions below:\n\n";
            fs::write(custom_prompt_path, default_content)?;
        }
        
        Ok(())
    }
}
