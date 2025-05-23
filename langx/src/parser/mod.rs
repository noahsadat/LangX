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
        .map_err(|e| format!("Parse error: {:?}", e))
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