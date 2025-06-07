# Quick Question (qq)
A simple, fast CLI for getting quick AI answers in your terminal.

I'm pretty new to the whole AI thing. I can see its value for sure. However, there's about 100 new super impressive do-everything or do-specific apps that come out daily. I found that I could use something a lot simpler to fill a niche. So I made this. It's designed to be as simple as possible.

- `qq is there a good wordpress hook for changing the menu structure if I have an id?`
- `qq "What's the difference between == and === in JavaScript?"`
- `qq "im working on a crossword, help? * * A * P"`

It's just to ask simple stuff. No file uploads or tons of iterating. No deep diving into your codebase to discover every known possible optimization. Just ask it a quick question.

## Prerequisites

- Rust 1.70+
- An Anthropic API key from [Anthropic](https://www.anthropic.com/)

## Installation

```bash
cargo install --git https://github.com/rumble19/quick-question.git
```

Or from source:
```bash
git clone https://github.com/rumble19/quick-question.git
cd quick-question
cargo install --path .
```

## Usage

### Interactive Mode (Recommended)

```bash
qq
# Enter your question when prompted
```

Interactive mode is the most practical way to use qq, especially for questions with special characters or longer queries.

### Direct Questions

```bash
qq "What is Rust?"
qq "How do I reverse a string in Python?"
```

### Pipe Input

```bash
echo "What is the capital of France?" | qq
```

## Configuration

Your configuration file will be created automatically on first run and is stored at:
- **Linux/macOS**: `~/.config/quick-question/config.toml`
- **Windows**: `%APPDATA%\quick-question\config.toml`

You will also be prompted to add your api key if one is not already set in the config file. 

Example config:
```toml
claude_api_key = "your-anthropic-api-key-here"
model = "claude-sonnet-4-20250514"
max_tokens = 300
```

You can also set your API key via environment variable:
```bash
export CLAUDE_API_KEY="your-anthropic-api-key-here"
```

## Roadmap

- ChatGPT/OpenAI support
- Local LLM support (Ollama, etc.)
- Offline history

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
