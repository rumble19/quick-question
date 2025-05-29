use clap::Parser;
use std::io::{self, Write};

mod config;
mod providers;

use config::Config;
use providers::{Provider, claude::ClaudeProvider};

#[derive(Parser)]
#[command(name = "qq")]
#[command(about = "Quick Question - Get fast answers in your terminal")]
struct Args {
    /// The question to ask
    question: Vec<String>,
    
    /// Run the setup process
    #[arg(long)]
    setup: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    if args.setup {
        setup_config().await?;
        return Ok(());
    }
    
    if args.question.is_empty() {
        eprintln!("Usage: qq <your question>");
        eprintln!("   or: qq --setup (to configure)");
        std::process::exit(1);
    }
    
    let question = args.question.join(" ");
    
    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            println!("🔧 First time setup needed!");
            setup_config().await?;
            Config::load()?
        }
    };
    
    let provider = ClaudeProvider::new(config.claude_api_key.clone(), config.model.clone());
    
    match provider.ask(&question).await {
        Ok(response) => println!("{}", response),
        Err(e) => {
            if e.to_string().contains("network") || e.to_string().contains("connection") {
                eprintln!("Sorry, I can't answer that without an active internet connection");
            } else if e.to_string().contains("token") || e.to_string().contains("quota") {
                eprintln!("Looks like you ran out of tokens, time to pay up.");
            } else if e.to_string().contains("401") || e.to_string().contains("authentication") {
                eprintln!("Authentication failed. Check your API key configuration.");
            } else {
                eprintln!("Something went wrong: {}", e);
            }
            std::process::exit(1);
        }
    }
    
    Ok(())
}

async fn setup_config() -> anyhow::Result<()> {
    println!("Welcome to Quick Question setup! 🚀");
    println!();
    
    print!("Please enter your Claude API key: ");
    io::stdout().flush()?;
    
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim().to_string();
    
    if api_key.is_empty() {
        eprintln!("API key cannot be empty.");
        std::process::exit(1);
    }
    
    let config = Config {
        claude_api_key: api_key,
        model: "claude-sonnet-4-20250514".to_string(),
        max_tokens: 300,
    };
    
    config.save()?;
    
    let config_path = Config::config_path()?;
    println!("✅ Configuration saved!");
    println!("📁 Config file: {}", config_path.display());
    println!("🔧 You can edit model and max_tokens settings there if needed.");
    println!();
    println!("Try it out: qq \"What is Rust?\"");
    
    Ok(())
}
