# Quick Question (qq) 🚀

A lightning-fast CLI tool that brings AI assistance directly to your terminal. Get quick, concise answers to any question without leaving your command line.

## Features

- ⚡ **Ultra-fast responses** - Get answers in seconds
- 🎯 **Concise output** - Responses optimized for terminal viewing (under 280 characters when possible)
- 🔧 **Easy setup** - One-time configuration with your Anthropic API key
- 🎨 **Beautiful** - Supports ASCII art and terminal formatting

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

Simply type `qq` followed by your question:

```bash
qq "What is Rust?"
qq "How do I reverse a string in Python?"
qq "Explain quantum computing simply"
qq "What's the weather API for OpenWeatherMap?"
```

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
$ qq "Rust ownership explained briefly"
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
├── Cargo.toml            # Dependencies and project config
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
- Inspired by the need for quick terminal-based AI assistance

---

**Happy questioning!** 🤔💭✨
