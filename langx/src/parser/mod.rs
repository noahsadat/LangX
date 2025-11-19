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
        let input = "If x is greater than 5, print \"Hello\".";
        let program = parse(input).unwrap();
        
        assert_eq!(program.statements.len(), 1);
        // Further assertions would check the structure of the parsed statement
    }
} 