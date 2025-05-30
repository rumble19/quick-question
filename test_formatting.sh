#!/bin/bash

# Test script to demonstrate markdown formatting without API calls

echo "🧪 Testing Markdown Formatting"
echo "================================"
echo

# Create a test response that mimics what Claude might return
test_response='Here is your test message with some goodies! 🚀

```
┌─────────────────────────────────┐
│    ASCII ART TEST ENGAGED!     │
└─────────────────────────────────┘
```

**FORMATTING TESTS:**
- *Italic text*
- **Bold text**
- `code snippet`
- ~~strikethrough~~

```
    /\_/\
   ( o.o )
    > ^ <
```

**Useful links for testing:**
• https://httpbin.org/json (returns test JSON)
• `curl https://httpbin.org/json`

Terminal formatting test:
🔴 **RED** 🟡 *YELLOW* 🟢 `GREEN` 🔵 ~~BLUE~~ 🟣 **PURPLE**

```
╔══════════════════════════════════╗
║  Hope this displays properly!    ║
║  With actual formatting! 😄      ║
╚══════════════════════════════════╝
```

*Edge case special chars: @#$%^&*()[]{}|;*'

echo "Expected output should show:"
echo "- Bold text in bright/bold formatting"  
echo "- Italic text in italic formatting"
echo "- Code snippets highlighted in yellow"
echo "- Strikethrough text with line through it"
echo "- Unicode box characters preserved"
echo

# We would need to modify the code to allow testing the formatter directly
# For now, this shows what the input looks like
echo "📝 Raw input (what Claude sends):"
echo "**Bold text** *italic text* \`code\` ~~strikethrough~~"
echo

echo "✨ Expected formatted output:"
echo -e "\033[1mBold text\033[0m \033[3mitalic text\033[0m \033[93mcode\033[0m \033[9mstrikethrough\033[0m"
