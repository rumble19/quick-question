use super::Provider;
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use std::fs;

const DEFAULT_SYSTEM_PROMPT: &str = "You are a helpful assistant designed to give quick, concise answers to terminal users. Keep responses under 280 characters when possible, but feel free to go a bit longer if necessary for clarity. Match the user's tone - if they ask something silly, be playful back. If they ask for facts, be matter-of-fact. Never ask follow-up questions or try to continue the conversation. When appropriate, include relevant links or sources. Use markdown formatting for emphasis: **bold**, *italic*, `code`, ~~strikethrough~~. Feel free to use ASCII art and Unicode characters - they display well in modern terminals. Remember: your response will be processed to show proper formatting in the terminal.

---

Anything after these instructions comes from the user.";

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
        
        let mut system_prompt = DEFAULT_SYSTEM_PROMPT.to_string();
        
        // Try to append custom prompt from user config directory
        if let Ok(config_dir) = Config::config_dir() {
            let custom_prompt_path = config_dir.join("custom_prompt.txt");
            if let Ok(custom_prompt) = fs::read_to_string(&custom_prompt_path) {
                let trimmed_custom = custom_prompt.trim();
                if !trimmed_custom.is_empty() && !trimmed_custom.starts_with("# Your custom prompt") {
                    system_prompt.push_str("\n\n");
                    system_prompt.push_str(trimmed_custom);
                }
            }
        }
        
        Ok(system_prompt)
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
            
        Ok(format_for_terminal(content))
    }
}

fn format_for_terminal(text: &str) -> String {
    let mut result = text.to_string();
    
    // Handle code blocks - remove ``` markers
    result = result.replace("```", "");
    
    // Convert Markdown bold to ANSI bold (**text**)
    result = replace_markdown_pattern(&result, "**", "\x1b[1m", "\x1b[0m");
    
    // Convert Markdown code spans to highlighted text (`code`)
    result = replace_markdown_pattern(&result, "`", "\x1b[93m", "\x1b[0m");
    
    // Convert strikethrough (~~text~~)
    result = replace_markdown_pattern(&result, "~~", "\x1b[9m", "\x1b[0m");
    
    // Convert Markdown italic to ANSI italic (*text*) - do this last to avoid conflicts
    result = replace_single_asterisk_italic(&result);
    
    result
}

fn replace_markdown_pattern(text: &str, marker: &str, start_ansi: &str, end_ansi: &str) -> String {
    let mut result = String::new();
    let chars = text.chars().collect::<Vec<_>>();
    let mut i = 0;
    
    while i < chars.len() {
        if i + marker.len() <= chars.len() && 
           chars[i..i + marker.len()].iter().collect::<String>() == marker {
            // Found opening marker, look for closing marker
            let mut j = i + marker.len();
            while j + marker.len() <= chars.len() {
                if chars[j..j + marker.len()].iter().collect::<String>() == marker {
                    // Found closing marker
                    let content: String = chars[i + marker.len()..j].iter().collect();
                    result.push_str(start_ansi);
                    result.push_str(&content);
                    result.push_str(end_ansi);
                    i = j + marker.len();
                    break;
                }
                j += 1;
            }
            if j + marker.len() > chars.len() {
                // No closing marker found, just add the character
                result.push(chars[i]);
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    
    result
}

fn replace_single_asterisk_italic(text: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '*' {
            // Check if it's not part of ** (already processed)
            let prev_is_asterisk = i > 0 && chars[i - 1] == '*';
            let next_is_asterisk = i + 1 < chars.len() && chars[i + 1] == '*';
            
            if !prev_is_asterisk && !next_is_asterisk {
                // Look for closing single asterisk
                let mut j = i + 1;
                while j < chars.len() {
                    if chars[j] == '*' && (j + 1 >= chars.len() || chars[j + 1] != '*') {
                        // Found closing single asterisk
                        let content: String = chars[i + 1..j].iter().collect();
                        result.push_str("\x1b[3m");
                        result.push_str(&content);
                        result.push_str("\x1b[0m");
                        i = j + 1;
                        break;
                    }
                    j += 1;
                }
                if j >= chars.len() {
                    // No closing found
                    result.push(chars[i]);
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    
    result
}
