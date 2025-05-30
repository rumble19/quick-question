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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    if args.setup {
        setup_config().await?;
        return Ok(());
    }
    
    let question = if args.question.is_empty() {
        // Check if we have stdin input (piped)
        if !IsTerminal::is_terminal(&io::stdin()) {
            // Read from stdin (pipe or redirection)
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            input.trim().to_string()
        } else {
            // No arguments and no pipe, go interactive
            get_question_interactively()?
        }
    } else {
        // Check if we might have gotten mangled input from shell
        let joined = args.question.join(" ");
        if looks_like_incomplete_input(&joined) {
            println!("🤔 It looks like your question might have been cut off by the shell.");
            println!("💡 Tip: Put quotes around questions with apostrophes or special characters:");
            println!("   qq \"your question here\"");
            println!("   Or just use: qq (and enter your question when prompted)");
            println!();
            print!("Enter your complete question: ");
            io::stdout().flush()?;
            let mut question = String::new();
            io::stdin().read_line(&mut question)?;
            question.trim().to_string()
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
    // Interactive terminal input only
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
    (input.len() < 5 && input.chars().any(|c| "'\"`\\".contains(c))) || // Very short with special chars
    // Check for patterns that suggest apostrophe was mangled by shell
    input.contains(" s ") ||     // "what s the" suggests "what's the"
    input.contains(" t ") ||     // "don t know" suggests "don't know"
    input.contains(" re ") ||    // "you re right" suggests "you're right"
    input.contains(" ll ") ||    // "we ll see" suggests "we'll see"
    input.contains(" ve ") ||    // "I ve got" suggests "I've got"
    input.contains(" d ") ||     // "I d like" suggests "I'd like"
    input.ends_with(" s") ||     // "what s" suggests "what's"
    input.ends_with(" t") ||     // "don t" suggests "don't"
    input.ends_with(" re") ||    // "you re" suggests "you're"
    input.ends_with(" ll") ||    // "we ll" suggests "we'll"
    input.ends_with(" ve") ||    // "I ve" suggests "I've"
    input.ends_with(" d")        // "I d" suggests "I'd"
}
