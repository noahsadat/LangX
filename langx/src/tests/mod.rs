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
            If x is greater than 5 then print \"x is large\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        
        // Just run the interpreter
        interpreter.interpret(&program).unwrap();
    }
    
    #[test]
    fn test_repeat() {
        let source = "Repeat 3 times: print \"Hello\".
End repeat.";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        
        // Just run the interpreter
        interpreter.interpret(&program).unwrap();
    }
    
    #[test]
    fn test_function() {
        let source = "
            Define add with parameters a, b:
                Return a + b.
            End definition.
            
            Set result to Call add with 5, 10.
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
            End while.
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
            End while.
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
            End while.
            print sum.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Sum of 1+2+3+4+5 = 15
        assert_eq!(interpreter.env.get("sum"), Some(crate::interpreter::Value::Number(15)));
    }
    
    // String concatenation tests
    #[test]
    fn test_string_concat_basic() {
        let source = "
            Set a to \"Hello\".
            Set b to \"World\".
            Set result to a + \", \" + b + \"!\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::String("Hello, World!".to_string()))
        );
    }
    
    #[test]
    fn test_string_concat_with_number() {
        let source = "
            Set text to \"The answer is \" + 42.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("text"),
            Some(crate::interpreter::Value::String("The answer is 42".to_string()))
        );
    }
    
    #[test]
    fn test_string_concat_number_first() {
        let source = "
            Set text to 100 + \" percent\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("text"),
            Some(crate::interpreter::Value::String("100 percent".to_string()))
        );
    }
    
    #[test]
    fn test_string_concat_with_boolean() {
        let source = "
            Set text to \"Status: \" + true.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("text"),
            Some(crate::interpreter::Value::String("Status: true".to_string()))
        );
    }
    
    // Multi-parameter function tests
    #[test]
    fn test_function_three_params() {
        let source = "
            Define add_three with parameters a, b, c:
                Return a + b + c.
            End definition.
            
            Set result to Call add_three with 1, 2, 3.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::Number(6))
        );
    }
    
    #[test]
    fn test_function_four_params() {
        let source = "
            Define multiply_four with parameters a, b, c, d:
                Return a * b * c * d.
            End definition.
            
            Set product to Call multiply_four with 2, 3, 4, 5.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("product"),
            Some(crate::interpreter::Value::Number(120))
        );
    }
    
    #[test]
    fn test_function_five_params() {
        let source = "
            Define sum_five with parameters a, b, c, d, e:
                Return a + b + c + d + e.
            End definition.
            
            Set total to Call sum_five with 1, 2, 3, 4, 5.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("total"),
            Some(crate::interpreter::Value::Number(15))
        );
    }
    
    // Edge case tests
    #[test]
    fn test_operator_precedence() {
        let source = "
            Set result to 2 + 3 * 4.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Should be 2 + (3 * 4) = 14, not (2 + 3) * 4 = 20
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::Number(14))
        );
    }
    
    #[test]
    fn test_parentheses_precedence() {
        let source = "
            Set result to (2 + 3) * 4.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Should be (2 + 3) * 4 = 20
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::Number(20))
        );
    }
    
    #[test]
    fn test_division_by_zero_error() {
        let source = "
            Set x to 10 / 0.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }
    
    #[test]
    fn test_undefined_variable_error() {
        let source = "
            Set x to undefined_var.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }
    
    #[test]
    fn test_undefined_function_error() {
        let source = "
            Set x to Call nonexistent with 1, 2.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined function"));
    }
    
    #[test]
    fn test_wrong_argument_count_error() {
        let source = "
            Define add with parameters a, b:
                Return a + b.
            End definition.
            
            Set x to Call add with 1, 2, 3.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 2 arguments"));
    }
    
    #[test]
    fn test_repeat_loop_with_variable() {
        let source = "
            Set count to 3.
            Set sum to 0.
            Repeat count times: Set sum to sum + 1.
            End repeat.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("sum"),
            Some(crate::interpreter::Value::Number(3))
        );
    }
    
    #[test]
    fn test_mixed_list_types() {
        let source = "
            Set mixed to [1, \"hello\", true, 42].
            Set num to item 0 of mixed.
            Set str to item 1 of mixed.
            Set bool to item 2 of mixed.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("num"),
            Some(crate::interpreter::Value::Number(1))
        );
        assert_eq!(
            interpreter.env.get("str"),
            Some(crate::interpreter::Value::String("hello".to_string()))
        );
        assert_eq!(
            interpreter.env.get("bool"),
            Some(crate::interpreter::Value::Boolean(true))
        );
    }
} 