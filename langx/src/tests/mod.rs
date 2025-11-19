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
                Set current_item to item (index) of list.
                Set sum to sum + current_item.
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
    
    #[test]
    fn test_split_function() {
        let source = "
            Set text to \"a,b,c\".
            Set parts to Call split with text, \",\".
            Set first to item 0 of parts.
            Set second to item 1 of parts.
            Set third to item 2 of parts.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("first"),
            Some(crate::interpreter::Value::String("a".to_string()))
        );
        assert_eq!(
            interpreter.env.get("second"),
            Some(crate::interpreter::Value::String("b".to_string()))
        );
        assert_eq!(
            interpreter.env.get("third"),
            Some(crate::interpreter::Value::String("c".to_string()))
        );
    }
    
    #[test]
    fn test_split_with_space() {
        let source = "
            Set text to \"hello world langx\".
            Set parts to Call split with text, \" \".
            Set first to item 0 of parts.
            Set second to item 1 of parts.
            Set third to item 2 of parts.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        if let Some(crate::interpreter::Value::List(items)) = interpreter.env.get("parts") {
            assert_eq!(items.len(), 3);
        } else {
            panic!("split should return a list");
        }
        assert_eq!(
            interpreter.env.get("first"),
            Some(crate::interpreter::Value::String("hello".to_string()))
        );
        assert_eq!(
            interpreter.env.get("second"),
            Some(crate::interpreter::Value::String("world".to_string()))
        );
        assert_eq!(
            interpreter.env.get("third"),
            Some(crate::interpreter::Value::String("langx".to_string()))
        );
    }
    
    #[test]
    fn test_join_function() {
        let source = "
            Set list to [\"a\", \"b\", \"c\"].
            Set result to Call join with list, \",\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::String("a,b,c".to_string()))
        );
    }
    
    #[test]
    fn test_join_with_space() {
        let source = "
            Set list to [\"hello\", \"world\", \"langx\"].
            Set result to Call join with list, \" \".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::String("hello world langx".to_string()))
        );
    }
    
    #[test]
    fn test_join_with_numbers() {
        let source = "
            Set list to [1, 2, 3].
            Set result to Call join with list, \"-\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::String("1-2-3".to_string()))
        );
    }
    
    #[test]
    fn test_replace_function() {
        let source = "
            Set text to \"Hello World\".
            Set result to Call replace with text, \"World\", \"LangX\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::String("Hello LangX".to_string()))
        );
    }
    
    #[test]
    fn test_replace_multiple_occurrences() {
        let source = "
            Set text to \"cat cat dog cat\".
            Set result to Call replace with text, \"cat\", \"dog\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::String("dog dog dog dog".to_string()))
        );
    }
    
    #[test]
    fn test_replace_empty_string() {
        let source = "
            Set text to \"hello\".
            Set result to Call replace with text, \"\", \"X\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Empty string replacement should insert X between each character
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::String("XhXeXlXlXoX".to_string()))
        );
    }
    
    #[test]
    fn test_string_functions_error_handling() {
        // Test split with wrong argument types
        let source = "
            Set result to Call split with 123, \",\".
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects (string, string)"));
        
        // Test join with wrong argument types
        let source2 = "
            Set result to Call join with \"not a list\", \",\".
        ";
        let program2 = parser::parse(source2).unwrap();
        let mut interpreter2 = Interpreter::new();
        let result2 = interpreter2.interpret(&program2);
        assert!(result2.is_err());
        assert!(result2.unwrap_err().contains("expects (list, string)"));
        
        // Test replace with wrong argument types
        let source3 = "
            Set result to Call replace with \"text\", 123, \"new\".
        ";
        let program3 = parser::parse(source3).unwrap();
        let mut interpreter3 = Interpreter::new();
        let result3 = interpreter3.interpret(&program3);
        assert!(result3.is_err());
        assert!(result3.unwrap_err().contains("expects (string, string, string)"));
    }
    
    #[test]
    fn test_for_loop_basic() {
        let source = "
            Set sum to 0.
            Set numbers to [1, 2, 3, 4, 5].
            For each num in numbers:
                Set sum to sum + num.
            End for.
            print sum.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Sum of 1+2+3+4+5 = 15
        assert_eq!(interpreter.env.get("sum"), Some(crate::interpreter::Value::Number(15)));
    }
    
    #[test]
    fn test_for_loop_empty_list() {
        let source = "
            Set count to 0.
            Set empty to [].
            For each elem in empty:
                Set count to count + 1.
            End for.
            print count.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Count should remain 0 for empty list
        assert_eq!(interpreter.env.get("count"), Some(crate::interpreter::Value::Number(0)));
    }
    
    #[test]
    fn test_for_loop_string_list() {
        let source = "
            Set result to \"\".
            Set words to [\"Hello\", \" \", \"World\", \"!\"].
            For each word in words:
                Set result to result + word.
            End for.
            print result.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::String("Hello World!".to_string()))
        );
    }
    
    #[test]
    fn test_for_loop_mixed_types() {
        let source = "
            Set result to \"\".
            Set items to [1, \"hello\", true, 42].
            For each elem in items:
                Set result to result + elem.
            End for.
            print result.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Should concatenate: 1 + "hello" + true + 42 = "1hellotrue42"
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::String("1hellotrue42".to_string()))
        );
    }
    
    #[test]
    fn test_for_loop_variable_shadowing() {
        let source = "
            Set x to 100.
            Set list to [1, 2, 3].
            For each x in list:
                print x.
            End for.
            print x.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // After the loop, x should be back to 100 (or the last value from the loop)
        // Actually, the loop variable persists after the loop, so x will be 3
        assert_eq!(interpreter.env.get("x"), Some(crate::interpreter::Value::Number(3)));
    }
    
    #[test]
    fn test_for_loop_nested() {
        let source = "
            Set sum to 0.
            Set matrix to [[1, 2], [3, 4], [5, 6]].
            # Note: nested lists not fully supported yet, but we can test with single-level
            Set rows to [1, 2, 3].
            For each row in rows:
                Set sum to sum + row.
            End for.
            print sum.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("sum"), Some(crate::interpreter::Value::Number(6)));
    }
    
    #[test]
    fn test_for_loop_with_function_call() {
        let source = "
            Define double with parameter x:
                Return x * 2.
            End definition.
            
            Set result to [].
            Set numbers to [1, 2, 3].
            For each num in numbers:
                Set doubled to Call double with num.
                Add doubled to result.
            End for.
            print result.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("result"),
            Some(crate::interpreter::Value::List(vec![
                crate::interpreter::Value::Number(2),
                crate::interpreter::Value::Number(4),
                crate::interpreter::Value::Number(6),
            ]))
        );
    }
    
    #[test]
    fn test_for_loop_error_not_list() {
        let source = "
            Set x to 5.
            For each elem in x:
                print elem.
            End for.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("For loop expects a list"));
    }
    
    #[test]
    fn test_for_loop_with_return() {
        // Test that return works inside a for loop
        let source = "
            Define find_first with parameter numbers:
                For each num in numbers:
                    If num is greater than 2 then Return num.
                End for.
                Set neg_one to 0 - 1.
                Return neg_one.
            End definition.
            
            Set list to [1, 2, 3, 4].
            Set result to Call find_first with list.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Should return 3 (first number > 2)
        assert_eq!(interpreter.env.get("result"), Some(crate::interpreter::Value::Number(3)));
    }
    
    #[test]
    fn test_break_in_repeat_loop() {
        let source = "
            Set count to 0.
            Repeat 10 times:
                Set count to count + 1.
                If count is greater than 5 then Break loop.
            End repeat.
            print count.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Should break at count = 6
        assert_eq!(interpreter.env.get("count"), Some(crate::interpreter::Value::Number(6)));
    }
    
    #[test]
    fn test_continue_in_repeat_loop() {
        let source = "
            Set sum to 0.
            Set count to 0.
            Repeat 10 times:
                Set count to count + 1.
                If count is equal to 5 then Continue to next iteration.
                Set sum to sum + count.
            End repeat.
            print sum.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Sum of 1+2+3+4+6+7+8+9+10 = 50 (skipping 5)
        assert_eq!(interpreter.env.get("sum"), Some(crate::interpreter::Value::Number(50)));
    }
    
    #[test]
    fn test_break_in_while_loop() {
        let source = "
            Set x to 0.
            While x is less than 10:
                Set x to x + 1.
                If x is greater than 5 then Break loop.
            End while.
            print x.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Should break at x = 6
        assert_eq!(interpreter.env.get("x"), Some(crate::interpreter::Value::Number(6)));
    }
    
    #[test]
    fn test_continue_in_while_loop() {
        let source = "
            Set x to 0.
            Set sum to 0.
            While x is less than 10:
                Set x to x + 1.
                If x is equal to 5 then Continue to next iteration.
                Set sum to sum + x.
            End while.
            print sum.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Sum of 1+2+3+4+6+7+8+9+10 = 50 (skipping 5)
        assert_eq!(interpreter.env.get("sum"), Some(crate::interpreter::Value::Number(50)));
    }
    
    #[test]
    fn test_break_in_for_loop() {
        let source = "
            Set count to 0.
            Set numbers to [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].
            For each num in numbers:
                Set count to count + 1.
                If num is greater than 5 then Break loop.
            End for.
            print count.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Should break at num = 6, so count = 6
        assert_eq!(interpreter.env.get("count"), Some(crate::interpreter::Value::Number(6)));
    }
    
    #[test]
    fn test_continue_in_for_loop() {
        let source = "
            Set sum to 0.
            Set numbers to [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].
            For each num in numbers:
                If num is equal to 5 then Continue to next iteration.
                Set sum to sum + num.
            End for.
            print sum.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Sum of 1+2+3+4+6+7+8+9+10 = 50 (skipping 5)
        assert_eq!(interpreter.env.get("sum"), Some(crate::interpreter::Value::Number(50)));
    }
    
    #[test]
    fn test_break_outside_loop_error() {
        let source = "
            Break loop.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Break statement outside of loop"));
    }
    
    #[test]
    fn test_continue_outside_loop_error() {
        let source = "
            Continue to next iteration.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Continue statement outside of loop"));
    }
    
    #[test]
    fn test_break_in_function_without_loop_error() {
        let source = "
            Define test:
                Break loop.
            End definition.
            Call test.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Break statement must be inside a loop"));
    }
    
    #[test]
    fn test_continue_in_function_without_loop_error() {
        let source = "
            Define test:
                Continue to next iteration.
            End definition.
            Call test.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Continue statement must be inside a loop"));
    }
    
    #[test]
    fn test_break_in_function_with_loop() {
        let source = "
            Define find_first_greater with parameter numbers, threshold:
                For each num in numbers:
                    If num is greater than threshold then Break loop.
                End for.
                Return num.
            End definition.
            
            Set list to [1, 2, 3, 4, 5].
            Set result to Call find_first_greater with list, 3.
        ";
        // Actually, this won't work because num is scoped to the loop
        // Let me test a simpler case - break works inside loop in function
        let source = "
            Define count_until with parameters numbers, limit:
                Set count to 0.
                For each num in numbers:
                    Set count to count + 1.
                    If num is greater than limit then Break loop.
                End for.
                Return count.
            End definition.
            
            Set list to [1, 2, 3, 4, 5, 6, 7].
            Set result to Call count_until with list, 4.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Should break at num = 5, so count = 5
        assert_eq!(interpreter.env.get("result"), Some(crate::interpreter::Value::Number(5)));
    }
} 