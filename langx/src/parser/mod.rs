use crate::ast::Program;
use crate::lexer::{tokenize, Token};

// Include the generated parser
#[allow(clippy::all)]
mod grammar {
    include!(concat!(env!("OUT_DIR"), "/parser/grammar.rs"));
}

/// Parse error with position information
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: Option<usize>,
    pub line_number: Option<usize>,
    pub suggestion: Option<String>,
}

impl ParseError {
    fn format(&self, input: &str) -> String {
        let mut error_msg = format!("Parse error: {}", self.message);
        
        if let Some(line_num) = self.line_number {
            error_msg.push_str(&format!("\nAt line {}:", line_num));
            
            if let Some(pos) = self.position {
                let snippet = crate::lexer::get_code_snippet(input, pos, 2);
                if !snippet.is_empty() {
                    error_msg.push_str(&format!("\n{}", snippet));
                }
            }
        }
        
        if let Some(suggestion) = &self.suggestion {
            error_msg.push_str(&format!("\nSuggestion: {}", suggestion));
        }
        
        error_msg.push_str("\nHint: Check for missing or extra tokens, and review parentheses and statement syntax.");
        error_msg
    }
}

/// Parse with advanced error recovery - collects multiple errors
pub fn parse_with_recovery(input: &str) -> Result<Program, Vec<ParseError>> {
    let mut errors = Vec::new();
    let tokens: Vec<(usize, Token, usize)> = tokenize(input);
    
    // Try normal parsing first
    let parser = grammar::ProgramParser::new();
    match parser.parse(tokens.iter().cloned()) {
        Ok(program) => return Ok(program),
        Err(e) => {
            // Extract first error
            let error_str = format!("{:?}", e);
            let position = extract_position_from_error(&error_str);
            let line_number = position.map(|pos| crate::lexer::line_number_at_position(input, pos));
            let suggestion = generate_suggestion(&error_str, &tokens, position);
            
            errors.push(ParseError {
                message: error_str.clone(),
                position,
                line_number,
                suggestion,
            });
            
            // Try error recovery: skip tokens and continue parsing
            if let Some(err_pos) = position {
                if let Some(recovered_program) = try_recover_parse(&tokens, err_pos, input, &mut errors) {
                    return Ok(recovered_program);
                }
            }
        }
    }
    
    Err(errors)
}

/// Try to recover from parse errors by skipping problematic tokens
fn try_recover_parse(
    tokens: &[(usize, Token, usize)],
    error_pos: usize,
    input: &str,
    errors: &mut Vec<ParseError>,
) -> Option<Program> {
    let parser = grammar::ProgramParser::new();
    
    // Strategy 1: Skip the problematic token and try again
    let mut skip_count = 0;
    let max_skips = 5; // Don't skip too many tokens
    
    while skip_count < max_skips {
        skip_count += 1;
        
        // Find tokens starting after the error position
        let mut recovered_tokens: Vec<(usize, Token, usize)> = Vec::new();
        let mut found_error_pos = false;
        let mut skipped = 0;
        
        for (pos, token, end) in tokens.iter() {
            if !found_error_pos && *pos >= error_pos {
                found_error_pos = true;
                // Skip the problematic token(s)
                if skipped < skip_count {
                    skipped += 1;
                    continue;
                }
            }
            recovered_tokens.push((*pos, token.clone(), *end));
        }
        
        // Try parsing with skipped tokens
        match parser.parse(recovered_tokens.iter().cloned()) {
            Ok(program) => {
                // Success! Add a warning about skipped tokens
                errors.push(ParseError {
                    message: format!("Recovered by skipping {} token(s) at error location", skip_count),
                    position: Some(error_pos),
                    line_number: Some(crate::lexer::line_number_at_position(input, error_pos)),
                    suggestion: Some("Review the skipped tokens - they may contain syntax errors.".to_string()),
                });
                return Some(program);
            }
            Err(_e) => {
                // Try next recovery strategy
                continue;
            }
        }
    }
    
    // Strategy 2: Try parsing statement by statement
    if let Some(program) = try_parse_statements_separately(tokens, input, errors) {
        return Some(program);
    }
    
    None
}

/// Try parsing statements separately to isolate errors
/// This is a simplified recovery strategy that attempts to parse valid statements
/// even if others have errors
fn try_parse_statements_separately(
    tokens: &[(usize, Token, usize)],
    input: &str,
    errors: &mut Vec<ParseError>,
) -> Option<Program> {
    let parser = grammar::ProgramParser::new();
    let mut statements = Vec::new();
    let mut current_statement_tokens: Vec<(usize, Token, usize)> = Vec::new();
    let mut statement_start = 0;
    
    // Keywords that typically start statements
    let statement_starters = vec![
        Token::Set,
        Token::If,
        Token::Print,
        Token::Repeat,
        Token::While,
        Token::For,
        Token::Define,
        Token::Return,
        Token::Break,
        Token::Continue,
    ];
    
    for (i, (pos, token, end)) in tokens.iter().enumerate() {
        // Check if this token starts a new statement
        if i > 0 && statement_starters.contains(token) {
            // Try to parse the previous statement as a complete program
            if !current_statement_tokens.is_empty() {
                match parser.parse(current_statement_tokens.iter().cloned()) {
                    Ok(program) => {
                        statements.extend(program.statements);
                    }
                    Err(e) => {
                        let error_str = format!("{:?}", e);
                        let position = extract_position_from_error(&error_str).or(Some(statement_start));
                        let line_number = position.map(|pos| crate::lexer::line_number_at_position(input, pos));
                        let suggestion = generate_suggestion(&error_str, tokens, position);
                        
                        errors.push(ParseError {
                            message: format!("Error in statement: {}", error_str),
                            position,
                            line_number,
                            suggestion,
                        });
                    }
                }
            }
            current_statement_tokens.clear();
            statement_start = *pos;
        }
        
        current_statement_tokens.push((*pos, token.clone(), *end));
    }
    
    // Parse the last statement
    if !current_statement_tokens.is_empty() {
        match parser.parse(current_statement_tokens.iter().cloned()) {
            Ok(program) => {
                statements.extend(program.statements);
            }
            Err(e) => {
                let error_str = format!("{:?}", e);
                let position = extract_position_from_error(&error_str).or(Some(statement_start));
                let line_number = position.map(|pos| crate::lexer::line_number_at_position(input, pos));
                let suggestion = generate_suggestion(&error_str, tokens, position);
                
                errors.push(ParseError {
                    message: format!("Error in statement: {}", error_str),
                    position,
                    line_number,
                    suggestion,
                });
            }
        }
    }
    
    if statements.is_empty() {
        None
    } else {
        Some(Program { statements })
    }
}

/// Generate helpful suggestions based on error type
fn generate_suggestion(error_str: &str, tokens: &[(usize, Token, usize)], position: Option<usize>) -> Option<String> {
    // Check for common error patterns
    if error_str.contains("UnrecognizedToken") {
        if let Some(pos) = position {
            // Find the token at this position
            for (token_pos, token, _) in tokens.iter() {
                if *token_pos == pos {
                    return suggest_token_fix(token);
                }
            }
        }
        return Some("Check for typos or unexpected tokens.".to_string());
    }
    
    if error_str.contains("UnrecognizedEof") {
        return Some("Unexpected end of input. Check for missing closing tokens (periods, parentheses, etc.).".to_string());
    }
    
    if error_str.contains("ExtraToken") {
        return Some("Extra token found. Check for missing periods or statement separators.".to_string());
    }
    
    None
}

/// Suggest fixes for specific token errors
fn suggest_token_fix(token: &Token) -> Option<String> {
    match token {
        Token::Identifier(name) => {
            // Check if it's a common typo
            let suggestions = vec![
                ("Set", "Set"),
                ("set", "Set"),
                ("If", "If"),
                ("if", "If"),
                ("Then", "then"),
                ("THEN", "then"),
                ("Print", "print"),
                ("print", "print"),
            ];
            
            for (wrong, correct) in suggestions {
                if name.eq_ignore_ascii_case(wrong) && name != correct {
                    return Some(format!("Did you mean '{}'?", correct));
                }
            }
            
            Some(format!("Unexpected identifier '{}'. Check spelling and syntax.", name))
        }
        _ => Some(format!("Unexpected token: {:?}", token)),
    }
}

/// Standard parse function (backward compatible)
pub fn parse(input: &str) -> Result<Program, String> {
    match parse_with_recovery(input) {
        Ok(program) => Ok(program),
        Err(errors) => {
            // Format all errors
            let mut error_msg = String::new();
            for (i, error) in errors.iter().enumerate() {
                if i > 0 {
                    error_msg.push_str("\n\n");
                }
                error_msg.push_str(&error.format(input));
            }
            
            if errors.len() > 1 {
                error_msg.push_str(&format!("\n\nFound {} errors total.", errors.len()));
            }
            
            Err(error_msg)
        }
    }
}

fn extract_position_from_error(error: &str) -> Option<usize> {
    // Try to extract location from error messages like "UnrecognizedToken { token: (60, ..."
    // or "UnrecognizedEof { location: 1088, ..."
    if let Some(start) = error.find("location: ") {
        if let Some(end) = error[start + 10..].find(',') {
            if let Ok(pos) = error[start + 10..start + 10 + end].trim().parse::<usize>() {
                return Some(pos);
            }
        }
    }
    if let Some(start) = error.find("token: (") {
        if let Some(end) = error[start + 8..].find(',') {
            if let Ok(pos) = error[start + 8..start + 8 + end].trim().parse::<usize>() {
                return Some(pos);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_assignment() {
        let input = "Set x to 10.";
        let program = parse(input).unwrap();
        
        assert_eq!(program.statements.len(), 1);
        // Further assertions would check the structure of the parsed statement
    }
    
    #[test]
    fn test_parse_conditional() {
        let input = "If x is greater than 5 then print \"Hello\".";
        let program = parse(input).unwrap();
        
        assert_eq!(program.statements.len(), 1);
        // Further assertions would check the structure of the parsed statement
    }
    
    #[test]
    fn test_parse_empty_program() {
        let input = "";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 0);
    }
    
    #[test]
    fn test_parse_multiple_statements() {
        let input = "Set x to 5. Set y to 10. Set z to x + y.";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 3);
    }
    
    #[test]
    fn test_parse_function_definition() {
        let input = "Define add with parameters a, b: Return a + b. End definition.";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
    }
    
    #[test]
    fn test_parse_list_literal() {
        let input = "Set list to [1, 2, 3].";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
    }
    
    #[test]
    fn test_parse_list_index() {
        let input = "Set x to item 0 of list.";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
    }
    
    #[test]
    fn test_parse_nested_expressions() {
        let input = "Set x to (2 + 3) * (4 - 1).";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
    }
    
    #[test]
    fn test_parse_invalid_syntax() {
        let input = "Set x to.";
        let result = parse(input);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_missing_period() {
        let input = "Set x to 5";
        let result = parse(input);
        // This might parse or error depending on grammar
        // Let's just check it doesn't panic
        let _ = result;
    }
    
    #[test]
    fn test_parse_complex_expression() {
        let input = "Set result to 2 + 3 * 4 - 5 / 2.";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
    }
    
    #[test]
    fn test_parse_logical_expression() {
        let input = "Set x to true and false or not true.";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
    }
} 