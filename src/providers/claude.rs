use super::Provider;
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use std::fs;

pub struct ClaudeProvider {
    api_key: String,
    model: String,
    client: Client,
}

impl ClaudeProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: Client::new(),
        }
    }
    
    fn get_system_prompt() -> Result<String> {
        use crate::config::Config;
        
        // Try custom prompt in user config directory first
        if let Ok(config_dir) = Config::config_dir() {
            let custom_prompt_path = config_dir.join("custom_prompt.txt");
            if let Ok(custom_prompt) = fs::read_to_string(&custom_prompt_path) {
                return Ok(custom_prompt.trim().to_string());
            }
        }
        
        // Fall back to default prompt in project directory
        let default_prompt_path = "prompts/system.txt";
        fs::read_to_string(default_prompt_path)
            .map(|s| s.trim().to_string())
            .map_err(|e| anyhow::anyhow!("Failed to read system prompt from {}: {}", default_prompt_path, e))
    }
}

#[async_trait::async_trait]
impl Provider for ClaudeProvider {
    async fn ask(&self, question: &str) -> Result<String> {
        let system_prompt = Self::get_system_prompt()?;
        
        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": self.model,
                "max_tokens": 300,
                "system": system_prompt,
                "messages": [
                    {
                        "role": "user",
                        "content": question
                    }
                ]
            }))
            .send()
            .await?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("API request failed with status {}: {}", status, error_text));
        }
        
        let json: Value = response.json().await?;
        
        let content = json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Unexpected response format"))?;
            
        Ok(content.to_string())
    }
}
