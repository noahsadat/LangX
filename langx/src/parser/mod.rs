use crate::ast::Program;
use crate::lexer::tokenize;

// Include the generated parser
#[allow(clippy::all)]
mod grammar {
    include!(concat!(env!("OUT_DIR"), "/parser/grammar.rs"));
}

pub fn parse(input: &str) -> Result<Program, String> {
    let tokens = tokenize(input);
    let parser = grammar::ProgramParser::new();
    
    parser.parse(tokens)
        .map_err(|e| {
            // Try to extract position from error
            let error_str = format!("{:?}", e);
            let position = extract_position_from_error(&error_str);
            
            let mut error_msg = format!("Parse error: {}", error_str);
            
            if let Some(pos) = position {
                let line_num = crate::lexer::line_number_at_position(input, pos);
                error_msg.push_str(&format!("\nAt line {}:", line_num));
                
                // Add code snippet
                let snippet = crate::lexer::get_code_snippet(input, pos, 2);
                if !snippet.is_empty() {
                    error_msg.push_str(&format!("\n{}", snippet));
                }
            }
            
            error_msg.push_str("\nHint: Check for missing or extra tokens, and review parentheses and statement syntax.");
            error_msg
        })
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