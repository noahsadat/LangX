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
    
    #[test]
    fn test_while_loop() {
        let source = "
            Set x to 0.
            While x is less than 3:
                Set x to x + 1.
            print x.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("x"), Some(crate::interpreter::Value::Number(3)));
    }
    
    #[test]
    fn test_while_loop_with_function_call() {
        let source = "
            Define is_positive with parameter n:
                Return n is greater than 0.
            End definition.
            
            Set x to 5.
            While Call is_positive with x:
                Set x to x - 1.
            print x.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("x"), Some(crate::interpreter::Value::Number(0)));
    }
    
    #[test]
    fn test_list_with_while_loop() {
        let source = "
            Set list to [1, 2, 3, 4, 5].
            Set index to 0.
            Set sum to 0.
            While index is less than 5:
                Set item to item (index) of list.
                Set sum to sum + item.
                Set index to index + 1.
            print sum.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Sum of 1+2+3+4+5 = 15
        assert_eq!(interpreter.env.get("sum"), Some(crate::interpreter::Value::Number(15)));
    }
} 