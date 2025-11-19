use logos::Logos;
use std::fmt;

/// Process escape sequences in a string literal
fn process_escape_sequences(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('0') => result.push('\0'),
                Some(c) => {
                    // Unknown escape sequence, keep as-is
                    result.push('\\');
                    result.push(c);
                }
                None => {
                    // Backslash at end of string, keep it
                    result.push('\\');
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

/// Parse a triple-quoted string manually
/// The lexer is at the position of the first " of """
fn parse_triple_quoted_string(lexer: &mut logos::Lexer<Token>) -> String {
    let source = lexer.source();
    let start = lexer.span().start;
    
    // We're at the first " of """, so the content starts at start + 3
    // Find the closing """
    let mut chars = source[start + 3..].char_indices();
    
    while let Some((idx, ch)) = chars.next() {
        if ch == '"' {
            // Check if this is the start of """
            let check_pos = start + 3 + idx;
            if check_pos + 2 < source.len() && &source[check_pos..check_pos + 3] == "\"\"\"" {
                // Found closing """
                let content = &source[start + 3..check_pos];
                // Update lexer position to after the closing """
                // The lexer is at start + 1 (after the first " matched by regex)
                // We need to advance to check_pos + 3
                // So bump by: (check_pos + 3) - (start + 1) = check_pos - start + 2
                lexer.bump(check_pos - start + 2);
                return process_escape_sequences(content);
            }
        }
    }
    
    // No closing """ found - return empty string (will cause parse error later)
    String::new()
}

/// Token types for the LangX language
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    #[token("Set")]
    Set,
    
    #[token("to")]
    To,
    
    #[token("If")]
    If,
    
    #[token("then")]
    Then,
    
    #[token("is")]
    Is,
    
    #[token("greater")]
    Greater,
    
    #[token("than")]
    Than,
    
    #[token("print")]
    Print,
    
    #[token("Repeat")]
    Repeat,
    
    #[token("While")]
    While,
    
    #[token("For")]
    For,
    
    #[token("each")]
    Each,
    
    #[token("in")]
    In,
    
    #[token("repeat")]
    RepeatLower,
    
    #[token("while")]
    WhileLower,
    
    #[token("for")]
    ForLower,
    
    #[token("times")]
    TimesKeyword,
    
    #[token("Define")]
    Define,
    
    #[token("with")]
    With,
    
    #[token("parameters")]
    Parameters,
    
    #[token("parameter")]
    Parameter,
    
    #[token("default")]
    Default,
    
    #[token("and")]
    And,
    
    #[token("End")]
    End,
    
    #[token("definition")]
    Definition,
    
    #[token("Call")]
    Call,
    
    #[token("Return")]
    Return,
    
    #[token("Break")]
    Break,
    
    #[token("Continue")]
    Continue,
    
    #[token("loop")]
    Loop,
    
    #[token("iteration")]
    Iteration,
    
    #[token("next")]
    Next,
    
    // Arithmetic operators (using symbols)
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Times,
    
    #[token("/")]
    Divide,
    
    // Punctuation
    #[token(".")]
    Period,
    
    #[token(",")]
    Comma,
    
    #[token(":")]
    Colon,
    
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    #[token("[")]
    LeftBracket,
    
    #[token("]")]
    RightBracket,
    
    #[token("{")]
    LeftBrace,
    
    #[token("}")]
    RightBrace,
    
    #[token("...")]
    Ellipsis,
    
    #[token("item")]
    Item,
    
    #[token("of")]
    Of,
    
    #[token("at")]
    At,
    
    #[token("Add")]
    Add,
    
    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok(), priority = 2)]
    Number(i64),
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 2)]
    Identifier(String),
    
    // String literals - handle both single and triple-quoted
    // Match " and check if it's triple-quoted or single-quoted
    #[regex(r#"""#, |lex| {
        let source = lex.source();
        let start = lex.span().start;
        
        // Check if this is the start of a triple-quoted string
        if start + 2 < source.len() && &source[start..start + 3] == "\"\"\"" {
            // Triple-quoted string - the regex matched the first ", but we need to handle all three
            // Reset to start and parse the full triple-quoted string
            parse_triple_quoted_string(lex)
        } else {
            // Single-quoted string - parse normally
            let remaining = &source[start + 1..];
            let mut chars = remaining.char_indices();
            let mut content = String::new();
            let mut escaped = false;
            
            while let Some((char_idx, ch)) = chars.next() {
                if escaped {
                    match ch {
                        'n' => content.push('\n'),
                        't' => content.push('\t'),
                        'r' => content.push('\r'),
                        '\\' => content.push('\\'),
                        '"' => content.push('"'),
                        '0' => content.push('\0'),
                        c => {
                            content.push('\\');
                            content.push(c);
                        }
                    }
                    escaped = false;
                } else if ch == '\\' {
                    escaped = true;
                } else if ch == '"' {
                    // Found closing quote
                    // char_idx is byte offset within remaining
                    // The lexer is at start + 1, we want to advance to start + 1 + char_idx + ch.len_utf8()
                    lex.bump(char_idx + ch.len_utf8());
                    return content;
                } else {
                    content.push(ch);
                }
            }
            
            // No closing quote found
            String::new()
        }
    }, priority = 3)]
    StringLiteral(String),
    
    // Comments (lines starting with #)
    #[regex(r"#[^\n]*", logos::skip)]
    Comment,
    
    // Whitespace
    #[regex(r"[ \t\n\f]+", logos::skip, priority = 1)]
    Whitespace,
    
    // Catch-all for errors
    #[regex(r".", logos::skip, priority = 0)]
    Error,
    
    #[token("less")]
    Less,
    
    #[token("equal")]
    Equal,
    
    #[token("not")]
    Not,
    
    #[token("or")]
    Or,
    
    // Boolean literals
    #[token("true")]
    True,
    
    #[token("false")]
    False,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Set => write!(f, "Set"),
            Token::To => write!(f, "to"),
            Token::If => write!(f, "If"),
            Token::Then => write!(f, "then"),
            Token::Is => write!(f, "is"),
            Token::Greater => write!(f, "greater"),
            Token::Than => write!(f, "than"),
            Token::Print => write!(f, "print"),
            Token::Repeat => write!(f, "Repeat"),
            Token::While => write!(f, "While"),
            Token::For => write!(f, "For"),
            Token::Each => write!(f, "each"),
            Token::In => write!(f, "in"),
            Token::RepeatLower => write!(f, "repeat"),
            Token::WhileLower => write!(f, "while"),
            Token::ForLower => write!(f, "for"),
            Token::TimesKeyword => write!(f, "times"),
            Token::Define => write!(f, "Define"),
            Token::With => write!(f, "with"),
            Token::Parameters => write!(f, "parameters"),
            Token::Parameter => write!(f, "parameter"),
            Token::And => write!(f, "and"),
            Token::End => write!(f, "End"),
            Token::Definition => write!(f, "definition"),
            Token::Call => write!(f, "Call"),
            Token::Return => write!(f, "Return"),
            Token::Break => write!(f, "Break"),
            Token::Continue => write!(f, "Continue"),
            Token::Loop => write!(f, "loop"),
            Token::Iteration => write!(f, "iteration"),
            Token::Next => write!(f, "next"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Times => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Period => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Item => write!(f, "item"),
            Token::Of => write!(f, "of"),
            Token::At => write!(f, "at"),
            Token::Add => write!(f, "Add"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::Error => write!(f, "ERROR"),
            Token::Whitespace => write!(f, " "),
            Token::Less => write!(f, "less"),
            Token::Equal => write!(f, "equal"),
            Token::Not => write!(f, "not"),
            Token::Or => write!(f, "or"),
            Token::Comment => write!(f, "#"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Default => write!(f, "default"),
            Token::Ellipsis => write!(f, "..."),
        }
    }
}

/// Tokenize a string into a vector of tokens with line numbers
pub fn tokenize(input: &str) -> Vec<(usize, Token, usize)> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();
    
    while let Some(token) = lexer.next() {
        if let Ok(token) = token {
            let span = lexer.span();
            tokens.push((span.start, token, span.end));
        }
    }
    
    tokens
}

/// Calculate line number from byte position in source code
pub fn line_number_at_position(source: &str, position: usize) -> usize {
    source[..position.min(source.len())]
        .chars()
        .filter(|&c| c == '\n')
        .count() + 1
}

/// Get a snippet of code around a position for error messages
pub fn get_code_snippet(source: &str, position: usize, context_lines: usize) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let line_num = line_number_at_position(source, position);
    
    if line_num == 0 || line_num > lines.len() {
        return format!("Line {}", line_num);
    }
    
    let start_line = (line_num.saturating_sub(context_lines + 1)).max(1);
    let end_line = (line_num + context_lines).min(lines.len());
    
    let mut snippet = String::new();
    for i in start_line..=end_line {
        let prefix = if i == line_num { "> " } else { "  " };
        snippet.push_str(&format!("{}{:3}: {}\n", prefix, i, lines[i - 1]));
    }
    
    snippet
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokenization() {
        let input = "Set x to 10.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("x".to_string()),
            Token::To,
            Token::Number(10),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_complex_tokenization() {
        let input = "If x is greater than 5, print \"Hello\".";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::If,
            Token::Identifier("x".to_string()),
            Token::Is,
            Token::Greater,
            Token::Than,
            Token::Number(5),
            Token::Comma,
            Token::Print,
            Token::StringLiteral("Hello".to_string()),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_function_tokenization() {
        let input = "Define add with parameters a and b:\nReturn a.\nEnd definition.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Define,
            Token::Identifier("add".to_string()),
            Token::With,
            Token::Parameters,
            Token::Identifier("a".to_string()),
            Token::And,
            Token::Identifier("b".to_string()),
            Token::Colon,
            Token::Return,
            Token::Identifier("a".to_string()),
            Token::Period,
            Token::End,
            Token::Definition,
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_arithmetic_tokenization() {
        let input = "Set x to 5 + 3 - 2 * 4 / 2.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("x".to_string()),
            Token::To,
            Token::Number(5),
            Token::Plus,
            Token::Number(3),
            Token::Minus,
            Token::Number(2),
            Token::Times,
            Token::Number(4),
            Token::Divide,
            Token::Number(2),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_parentheses_tokenization() {
        let input = "Set x to (5 + 3) * 2.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("x".to_string()),
            Token::To,
            Token::LeftParen,
            Token::Number(5),
            Token::Plus,
            Token::Number(3),
            Token::RightParen,
            Token::Times,
            Token::Number(2),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_empty_string_tokenization() {
        let input = "Set x to \"\".";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("x".to_string()),
            Token::To,
            Token::StringLiteral("".to_string()),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_string_with_spaces() {
        let input = "Set x to \"hello world\".";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("x".to_string()),
            Token::To,
            Token::StringLiteral("hello world".to_string()),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_boolean_literals() {
        let input = "Set x to true. Set y to false.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("x".to_string()),
            Token::To,
            Token::True,
            Token::Period,
            Token::Set,
            Token::Identifier("y".to_string()),
            Token::To,
            Token::False,
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_comments_skipped() {
        let input = "# This is a comment\nSet x to 5.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        // Comment should be skipped
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("x".to_string()),
            Token::To,
            Token::Number(5),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_list_brackets() {
        let input = "Set list to [1, 2, 3].";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("list".to_string()),
            Token::To,
            Token::LeftBracket,
            Token::Number(1),
            Token::Comma,
            Token::Number(2),
            Token::Comma,
            Token::Number(3),
            Token::RightBracket,
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_logical_operators() {
        let input = "Set x to true and false or not true.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("x".to_string()),
            Token::To,
            Token::True,
            Token::And,
            Token::False,
            Token::Or,
            Token::Not,
            Token::True,
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_comparison_keywords() {
        let input = "If x is greater than 5 then Set y to 10.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::If,
            Token::Identifier("x".to_string()),
            Token::Is,
            Token::Greater,
            Token::Than,
            Token::Number(5),
            Token::Then,
            Token::Set,
            Token::Identifier("y".to_string()),
            Token::To,
            Token::Number(10),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_break_continue_keywords() {
        let input = "Break loop. Continue to next iteration.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Break,
            Token::Loop,
            Token::Period,
            Token::Continue,
            Token::To,
            Token::Next,
            Token::Iteration,
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_for_loop_keywords() {
        let input = "For each elem in list: print elem. End for.";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::For,
            Token::Each,
            Token::Identifier("elem".to_string()),
            Token::In,
            Token::Identifier("list".to_string()),
            Token::Colon,
            Token::Print,
            Token::Identifier("elem".to_string()),
            Token::Period,
            Token::End,
            Token::ForLower,
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_multiline_string_basic() {
        let input = "Set text to \"\"\"Hello\nWorld\"\"\".";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("text".to_string()),
            Token::To,
            Token::StringLiteral("Hello\nWorld".to_string()),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_multiline_string_with_quotes() {
        let input = "Set text to \"\"\"Say \"Hello\" to me\"\"\".";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("text".to_string()),
            Token::To,
            Token::StringLiteral("Say \"Hello\" to me".to_string()),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_multiline_string_empty() {
        let input = "Set text to \"\"\"\"\"\".";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("text".to_string()),
            Token::To,
            Token::StringLiteral("".to_string()),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_multiline_string_with_escape_sequences() {
        let input = "Set text to \"\"\"Line 1\\nLine 2\\tTabbed\"\"\".";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("text".to_string()),
            Token::To,
            Token::StringLiteral("Line 1\nLine 2\tTabbed".to_string()),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_multiline_string_vs_single_quoted() {
        // Test that """ is parsed as triple-quoted, not as three single-quoted strings
        let input = "Set text1 to \"single\". Set text2 to \"\"\"triple\"\"\".";
        let tokens: Vec<Token> = tokenize(input).into_iter().map(|(_, t, _)| t).collect();
        
        assert_eq!(tokens, vec![
            Token::Set,
            Token::Identifier("text1".to_string()),
            Token::To,
            Token::StringLiteral("single".to_string()),
            Token::Period,
            Token::Set,
            Token::Identifier("text2".to_string()),
            Token::To,
            Token::StringLiteral("triple".to_string()),
            Token::Period,
        ]);
    }
} 