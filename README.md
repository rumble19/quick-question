# Quick Question (qq) 🚀

I'm pretty new to the whole AI thing. I can see its value for sure. However, there's about 100 new super impresive do-everything or do-specific apps that come out daily. I found that I could use something a <del>little</del> lot simpler to fill a niche. So I made this. It's designed to be as simple as possible. 
- `qq is there a good wordpress hook for changing the menu structure if I have an id?`
- `qq "What's the difference between == and === in JavaScript?"`
- `qq "im working on a crossword, help? * * A * P"`

It's just to ask simple stuff. No file uploads or tons of iterating. No deep diving into your codebase to discover every known possible optimization. Just ask it a quick question. 

## Features

- ⚡ **Ultra-fast responses** - Get answers in seconds
- 🎯 **Concise output** - Responses optimized for terminal viewing
- 🔧 **Easy setup** - One-time configuration with your Anthropic API key. More model support is planned. 
- 🎨 **Beautiful** - Supports ASCII art and terminal formatting
- 🙏 **Easy to use** - Doesn't have a million featues. Type qq and go. 

## Installation

### From Source

1. Clone the repository:
```bash
git clone https://github.com/rumble19/quick-question.git
cd quick-question
```

2. Build and install:
```bash
cargo build --release
cargo install --path .
```

3. Run the setup:
```bash
qq --setup
```

### Prerequisites

- Rust 1.70+ 
- An Anthropic API key from [Anthropic](https://www.anthropic.com/)

## Usage

### First Time Setup

Run the setup command and enter your Anthropic API key:

```bash
qq --setup
```

### Asking Questions

You have several ways to ask questions:

**1. Direct command (quoted):** Good if your query has punctuation that might mess with the input. 
```bash
qq "What is Rust?"
qq "How do I reverse a string in Python?"
qq "Explain quantum computing simply"
```

**2. Interactive mode:**  Also good for longer, more complex queries. 
```bash
qq -i
# Or just:
qq
# (will prompt for input if no arguments provided)
```

**3. Pipe input:**
```bash
echo "What is the capital of France?" | qq
```

**💡 Tip for Complex Questions:**
If your question contains apostrophes, quotes, or special characters, you have options:
- Use quotes: `qq "What's the difference between Rust and C++?"`
- Use interactive mode: `qq -i` (then type your question)
- Pipe it: `echo "Your complex question here" | qq`

The app will automatically detect problematic input and offer to switch to interactive mode!

### Configuration

Your configuration is stored at:
- **Linux/macOS**: `~/.config/quick-question/config.toml`
- **Windows**: `%APPDATA%\quick-question\config.toml`

You can edit this file to customize:
- `model`: The Claude model to use (default: "claude-sonnet-4-20250514")
- `max_tokens`: Maximum response length (default: 300)

Example config:
```toml
claude_api_key = "your-anthropic-api-key-here"
model = "claude-sonnet-4-20250514"
max_tokens = 300
```

## Customizing Prompts

You can customize the system prompt by creating a `custom_prompt.txt` file in your configuration directory. The application comes with a built-in default prompt optimized for concise terminal responses, but you can override it for your specific needs.

To create a custom prompt:
```bash
# Create the config directory if it doesn't exist  
mkdir -p ~/.config/quick-question

# Create and edit your custom prompt file
nano ~/.config/quick-question/custom_prompt.txt
```

The custom prompt file is stored in your user configuration directory:
- **Linux/macOS**: `~/.config/quick-question/custom_prompt.txt`  
- **Windows**: `%APPDATA%\quick-question\custom_prompt.txt`

If no custom prompt file is found, the application uses its built-in default prompt.

### Environment Variables

You can also set your API key via environment variable:
```bash
export CLAUDE_API_KEY="your-anthropic-api-key-here"
```

## Examples

```bash
# Quick facts
$ qq "Capital of Japan"
Tokyo is the capital of Japan. 🗾

# Programming help
$ qq "How does ownership work in rust?"
Ownership in Rust: each value has one owner, no garbage collector needed. 
When owner goes out of scope, value is dropped. Borrowing lets you use 
values without taking ownership. Prevents memory leaks & data races! 🦀

# Terminal tips
$ qq "How to find large files in Linux"
Use: find /path -type f -size +100M -exec ls -lh {} \;
Or: du -h /path | sort -hr | head -10
💡 Replace 100M with your size threshold!
```

## Project Structure

```
quick-question/
├── src/
│   ├── main.rs           # CLI interface and main logic
│   ├── config.rs         # Configuration management
│   └── providers/
│       ├── mod.rs        # Provider trait definition
│       └── claude.rs     # Claude API implementation
├── tests/                # Test files and demos
│   ├── demo_input_handling.sh  # Input method demonstrations
│   ├── test_formatting.sh      # Formatting tests
│   └── format_test.rs           # Standalone formatting test
├── Cargo.toml            # Dependencies and project config
├── LICENSE               # MIT License
└── README.md             # This file
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
cargo run -- "your question here"
cargo run -- --setup
```

## Dependencies

- **tokio**: Async runtime
- **reqwest**: HTTP client for API calls
- **clap**: Command-line argument parsing
- **serde**: Serialization/deserialization
- **anyhow**: Error handling
- **dirs**: Cross-platform config directory detection

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Claude](https://www.anthropic.com/) by Anthropic
- Inspired by the need for something simpler.

---

**Happy questioning!** 🤔💭✨
