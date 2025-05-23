use logos::Logos;
use std::fmt;

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
    
    #[token("times")]
    Times,
    
    #[token("Define")]
    Define,
    
    #[token("with")]
    With,
    
    #[token("parameters")]
    Parameters,
    
    #[token("parameter")]
    Parameter,
    
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
    
    // Arithmetic operators
    #[token("plus")]
    Plus,
    
    #[token("minus")]
    Minus,
    
    #[token("divided")]
    Divided,
    
    #[token("by")]
    By,
    
    // Punctuation
    #[token(".")]
    Period,
    
    #[token(",")]
    Comma,
    
    #[token(":")]
    Colon,
    
    #[token("\"")]
    Quote,
    
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok(), priority = 2)]
    Number(i64),
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 2)]
    Identifier(String),
    
    #[regex(r#""[^"]*""#, |lex| {
        let slice = lex.slice();
        // Remove the quotes
        slice[1..slice.len()-1].to_string()
    }, priority = 2)]
    StringLiteral(String),
    
    // Whitespace and comments
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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Set => write!(f, "Set"),
            Token::To => write!(f, "to"),
            Token::If => write!(f, "If"),
            Token::Is => write!(f, "is"),
            Token::Greater => write!(f, "greater"),
            Token::Than => write!(f, "than"),
            Token::Print => write!(f, "print"),
            Token::Repeat => write!(f, "Repeat"),
            Token::Times => write!(f, "times"),
            Token::Define => write!(f, "Define"),
            Token::With => write!(f, "with"),
            Token::Parameters => write!(f, "parameters"),
            Token::Parameter => write!(f, "parameter"),
            Token::And => write!(f, "and"),
            Token::End => write!(f, "End"),
            Token::Definition => write!(f, "definition"),
            Token::Call => write!(f, "Call"),
            Token::Return => write!(f, "Return"),
            Token::Plus => write!(f, "plus"),
            Token::Minus => write!(f, "minus"),
            Token::Divided => write!(f, "divided"),
            Token::By => write!(f, "by"),
            Token::Period => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Quote => write!(f, "\""),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::Error => write!(f, "ERROR"),
            Token::Whitespace => write!(f, " "),
            Token::Less => write!(f, "less"),
            Token::Equal => write!(f, "equal"),
            Token::Not => write!(f, "not"),
            Token::Or => write!(f, "or"),
        }
    }
}

/// Tokenize a string into a vector of tokens
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
        let input = "Set x to 5 plus 3 minus 2 times 4 divided by 2.";
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
            Token::Divided,
            Token::By,
            Token::Number(2),
            Token::Period,
        ]);
    }
    
    #[test]
    fn test_parentheses_tokenization() {
        let input = "Set x to (5 plus 3) times 2.";
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
} 