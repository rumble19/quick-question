use std::io::{self, Write};

// Copy the formatting functions here for testing
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

fn main() {
    let test_input = "Here's a test: **bold text**, *italic text*, `code block`, and ~~strikethrough~~!";
    
    println!("🧪 Formatting Test");
    println!("==================");
    println!();
    println!("Input:  {}", test_input);
    println!("Output: {}", format_for_terminal(test_input));
    println!();
    println!("✅ If you see different formatting above, it's working!");
}
