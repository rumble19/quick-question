use clap::Parser;
use std::io::{self, Write, IsTerminal, Read};

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
    
    /// Interactive mode - prompt for question
    #[arg(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    if args.setup {
        setup_config().await?;
        return Ok(());
    }
    
    let question = if args.interactive || args.question.is_empty() {
        get_question_interactively()?
    } else {
        // Check if we might have gotten mangled input from shell
        let joined = args.question.join(" ");
        if looks_like_incomplete_input(&joined) {
            println!("🤔 It looks like your question might have been cut off by the shell.");
            println!("💡 Tip: Put quotes around questions with apostrophes or special characters:");
            println!("   qq \"your question here\"");
            println!("   Or use interactive mode: qq -i");
            println!();
            print!("Enter your complete question: ");
            io::stdout().flush()?;
            get_question_interactively()?
        } else {
            joined
        }
    };
    
    if question.trim().is_empty() {
        println!("❌ No question provided.");
        return Ok(());
    }

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

fn get_question_interactively() -> anyhow::Result<String> {
    // Check if we're reading from a pipe or terminal
    if !IsTerminal::is_terminal(&io::stdin()) {
        // Read from stdin (pipe or redirection)
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        return Ok(input.trim().to_string());
    }
    
    // Interactive terminal input
    print!("❓ Enter your question: ");
    io::stdout().flush()?;
    
    let mut question = String::new();
    io::stdin().read_line(&mut question)?;
    Ok(question.trim().to_string())
}

fn looks_like_incomplete_input(input: &str) -> bool {
    // Check for common patterns that indicate shell mangling
    input.ends_with("'") ||  // Unclosed single quote
    input.ends_with('"') ||  // Unclosed double quote  
    input.ends_with("\\") || // Trailing backslash
    input.is_empty() ||      // Empty input
    (input.len() < 5 && input.chars().any(|c| "'\"`\\".contains(c))) // Very short with special chars
}
