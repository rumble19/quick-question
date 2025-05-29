use super::Provider;
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

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
    
    fn get_system_prompt() -> &'static str {
        "You are a helpful assistant designed to give quick, concise answers to terminal users. \
        Keep responses under 280 characters when possible, but feel free to go a bit longer if necessary for clarity. \
        Match the user's tone - if they ask something silly, be playful back. If they ask for facts, be matter-of-fact. \
        Never ask follow-up questions or try to continue the conversation. \
        When appropriate, include relevant links or sources. \
        Feel free to use ASCII art or terminal-friendly formatting when it adds value. \
        Remember: your response will be displayed directly in a terminal."
    }
}

#[async_trait::async_trait]
impl Provider for ClaudeProvider {
    async fn ask(&self, question: &str) -> Result<String> {
        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": self.model,
                "max_tokens": 300,
                "system": Self::get_system_prompt(),
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
