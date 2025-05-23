#[cfg(test)]
mod integration_tests {
    use crate::parser;
    use crate::interpreter::Interpreter;
    
    #[test]
    fn test_variable_assignment() {
        let source = "Set x to 42. print x.";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        
        // Just run the interpreter without capturing output
        interpreter.interpret(&program).unwrap();
    }
    
    #[test]
    fn test_conditional() {
        let source = "
            Set x to 10.
            If x is greater than 5, print \"x is large\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        
        // Just run the interpreter
        interpreter.interpret(&program).unwrap();
    }
    
    #[test]
    fn test_repeat() {
        let source = "Repeat 3 times: print \"Hello\".";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        
        // Just run the interpreter
        interpreter.interpret(&program).unwrap();
    }
    
    #[test]
    fn test_function() {
        let source = "
            Define add with parameters a and b:
                Return a.
            End definition.
            
            Set result to Call add with 5 and 10.
            print result.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        
        // Just run the interpreter
        interpreter.interpret(&program).unwrap();
    }
    
    #[test]
    fn test_from_file() {
        let file_path = "examples/hello.lx";
        let source = std::fs::read_to_string(file_path).unwrap();
        let program = parser::parse(&source).unwrap();
        let mut interpreter = Interpreter::new();
        
        let result = interpreter.interpret(&program);
        assert!(result.is_ok());
    }
} 