use crate::ast::{Program, Statement, Expression, BinaryOperator, UnaryOperator};
use std::collections::HashMap;

/// Value types that can be stored in variables
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Null => write!(f, "null"),
        }
    }
}

/// A function definition
#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Vec<Statement>,
}

/// Environment for storing variables and functions
#[derive(Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        match self.variables.get(name) {
            Some(value) => Some(value.clone()),
            None => {
                if let Some(parent) = &self.parent {
                    parent.get(name)
                } else {
                    None
                }
            }
        }
    }
    
    pub fn set(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }
    
    pub fn define_function(&mut self, name: &str, function: Function) {
        self.functions.insert(name.to_string(), function);
    }
    
    pub fn get_function(&self, name: &str) -> Option<Function> {
        match self.functions.get(name) {
            Some(function) => Some(function.clone()),
            None => {
                if let Some(parent) = &self.parent {
                    parent.get_function(name)
                } else {
                    None
                }
            }
        }
    }
}

/// Result of executing a statement
#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Normal,
    Return(Option<Value>),
}

/// Interpreter for executing LangX programs
pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        
        // Register built-in string functions
        Self::register_builtin_functions(&mut env);
        
        Self { env }
    }
    
    fn register_builtin_functions(_env: &mut Environment) {
        // Built-in functions are handled directly in handle_builtin_function
        // No need to register them as regular functions
    }
    
    pub fn interpret(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.statements {
            match self.execute_statement(statement)? {
                ExecutionResult::Normal => {},
                ExecutionResult::Return(value) => {
                    // Top-level return is not allowed
                    return Err(format!("Return statement outside of function: {:?}", value));
                }
            }
        }
        Ok(())
    }
    
    fn execute_statement(&mut self, statement: &Statement) -> Result<ExecutionResult, String> {
        match statement {
            Statement::Assignment { variable, value } => {
                let evaluated_value = self.evaluate_expression(value)?;
                self.env.set(variable, evaluated_value);
                Ok(ExecutionResult::Normal)
            }
            Statement::Conditional { condition, then_branch } => {
                let condition_value = self.evaluate_expression(condition)?;
                if let Value::Boolean(true) = condition_value {
                    self.execute_statement(then_branch)
                } else {
                    Ok(ExecutionResult::Normal)
                }
            }
            Statement::Print(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("{}", value);
                Ok(ExecutionResult::Normal)
            }
            Statement::Repeat { count, body } => {
                let count_value = self.evaluate_expression(count)?;
                if let Value::Number(n) = count_value {
                    for _ in 0..n {
                        match self.execute_statement(body)? {
                            ExecutionResult::Normal => {},
                            result @ ExecutionResult::Return(_) => return Ok(result),
                        }
                    }
                    Ok(ExecutionResult::Normal)
                } else {
                    Err(format!("Expected number for repeat count, got {:?}", count_value))
                }
            }
            Statement::While { condition, body } => {
                loop {
                    // Re-evaluate condition each iteration to get updated variable values
                    let condition_value = self.evaluate_expression(condition)?;
                    if let Value::Boolean(false) = condition_value {
                        break;
                    }
                    if let Value::Boolean(true) = condition_value {
                        match self.execute_statement(body)? {
                            ExecutionResult::Normal => {},
                            result @ ExecutionResult::Return(_) => return Ok(result),
                        }
                    } else {
                        return Err(format!("While loop condition must evaluate to a boolean, got {:?}", condition_value));
                    }
                }
                Ok(ExecutionResult::Normal)
            }
            Statement::Block(statements) => {
                for stmt in statements {
                    match self.execute_statement(stmt)? {
                        ExecutionResult::Normal => {},
                        result @ ExecutionResult::Return(_) => return Ok(result),
                    }
                }
                Ok(ExecutionResult::Normal)
            }
            Statement::FunctionDefinition { name, parameters, body } => {
                let function = Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                };
                self.env.define_function(name, function);
                Ok(ExecutionResult::Normal)
            }
            Statement::Return(expr) => {
                let value = match expr {
                    Some(e) => Some(self.evaluate_expression(e)?),
                    None => None,
                };
                Ok(ExecutionResult::Return(value))
            }
            Statement::ListAppend { list_name, value } => {
                let list_value = self.env.get(list_name)
                    .ok_or_else(|| format!("Runtime error: Undefined variable '{}'.", list_name))?;
                
                if let Value::List(mut items) = list_value {
                    let value_to_add = self.evaluate_expression(value)?;
                    items.push(value_to_add);
                    self.env.set(list_name, Value::List(items));
                    Ok(ExecutionResult::Normal)
                } else {
                    Err(format!("Runtime error: Variable '{}' is not a list.", list_name))
                }
            }
        }
    }
    
    fn evaluate_expression(&self, expr: &Expression) -> Result<Value, String> {
        // The parser already handles precedence correctly through the grammar structure
        // (Term for +/-, Factor for */), so we can evaluate directly
        self.evaluate_expression_internal(expr)
    }
    
    fn handle_builtin_function(&self, name: &str, arguments: &[Expression]) -> Result<Option<Value>, String> {
        // Evaluate arguments first
        let mut arg_values = Vec::new();
        for arg in arguments {
            arg_values.push(self.evaluate_expression(arg)?);
        }
        
        match name {
            "string_length" => {
                if arg_values.len() != 1 {
                    return Err(format!("Built-in function 'string_length' expects 1 argument, got {}.", arg_values.len()));
                }
                if let Value::String(s) = &arg_values[0] {
                    Ok(Some(Value::Number(s.len() as i64)))
                } else {
                    Err(format!("Built-in function 'string_length' expects a string argument, got {:?}.", arg_values[0]))
                }
            }
            "substring" => {
                if arg_values.len() != 3 {
                    return Err(format!("Built-in function 'substring' expects 3 arguments (string, start, length), got {}.", arg_values.len()));
                }
                if let (Value::String(s), Value::Number(start), Value::Number(len)) = (&arg_values[0], &arg_values[1], &arg_values[2]) {
                    if *start < 0 || *len < 0 {
                        return Err("Built-in function 'substring' requires non-negative start and length.".to_string());
                    }
                    let start_usize = *start as usize;
                    let len_usize = *len as usize;
                    if start_usize > s.len() {
                        return Err(format!("Start index {} is out of bounds for string of length {}.", start_usize, s.len()));
                    }
                    let end = (start_usize + len_usize).min(s.len());
                    Ok(Some(Value::String(s[start_usize..end].to_string())))
                } else {
                    Err(format!("Built-in function 'substring' expects (string, number, number) arguments, got ({:?}, {:?}, {:?}).", 
                        arg_values[0], arg_values[1], arg_values[2]))
                }
            }
            _ => Ok(None), // Not a built-in function
        }
    }
    
    fn evaluate_expression_internal(&self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Number(n) => Ok(Value::Number(*n)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Boolean(b) => Ok(Value::Boolean(*b)),
            Expression::Variable(name) => {
                self.env.get(name)
                    .ok_or_else(|| format!(
                        "Runtime error: Undefined variable '{}'.\nHint: Check if the variable is declared and spelled correctly.",
                        name
                    ))
            }
            Expression::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expression_internal(left)?;
                let right_val = self.evaluate_expression_internal(right)?;
                match operator {
                    BinaryOperator::GreaterThan => {
                        if let (Value::Number(l), Value::Number(r)) = (&left_val, &right_val) {
                            Ok(Value::Boolean(l > r))
                        } else {
                            Err(format!("Cannot compare {:?} and {:?}", left_val, right_val))
                        }
                    }
                    BinaryOperator::LessThan => {
                        if let (Value::Number(l), Value::Number(r)) = (&left_val, &right_val) {
                            Ok(Value::Boolean(l < r))
                        } else {
                            Err(format!("Cannot compare {:?} and {:?}", left_val, right_val))
                        }
                    }
                    BinaryOperator::Equal => {
                        Ok(Value::Boolean(left_val == right_val))
                    }
                    BinaryOperator::NotEqual => {
                        Ok(Value::Boolean(left_val != right_val))
                    }
                    BinaryOperator::Plus => {
                        match (&left_val, &right_val) {
                            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                            (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                            (Value::String(l), Value::Number(r)) => Ok(Value::String(format!("{}{}", l, r))),
                            (Value::Number(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                            (Value::String(l), Value::Boolean(r)) => Ok(Value::String(format!("{}{}", l, r))),
                            (Value::Boolean(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                            _ => Err(format!(
                                "Runtime error: Cannot add {:?} and {:?}.\nHint: Use + for numbers or string concatenation.",
                                left_val, right_val
                            ))
                        }
                    }
                    BinaryOperator::Minus => {
                        if let (Value::Number(l), Value::Number(r)) = (&left_val, &right_val) {
                            Ok(Value::Number(l - r))
                        } else {
                            Err(format!(
                                "Runtime error: Cannot subtract {:?} from {:?}.\nHint: Both operands must be numbers.",
                                right_val, left_val
                            ))
                        }
                    }
                    BinaryOperator::Times => {
                        if let (Value::Number(l), Value::Number(r)) = (&left_val, &right_val) {
                            Ok(Value::Number(l * r))
                        } else {
                            Err(format!(
                                "Runtime error: Cannot multiply {:?} and {:?}.\nHint: Both operands must be numbers.",
                                left_val, right_val
                            ))
                        }
                    }
                    BinaryOperator::Divide => {
                        if let (Value::Number(l), Value::Number(r)) = (&left_val, &right_val) {
                            if *r == 0 {
                                Err("Runtime error: Division by zero.\nHint: The divisor must not be zero.".to_string())
                            } else {
                                Ok(Value::Number(l / r))
                            }
                        } else {
                            Err(format!(
                                "Runtime error: Cannot divide {:?} by {:?}.\nHint: Both operands must be numbers.",
                                left_val, right_val
                            ))
                        }
                    }
                    BinaryOperator::And => {
                        if let Value::Boolean(l) = left_val {
                            if !l {
                                return Ok(Value::Boolean(false));
                            }
                            let right_val = self.evaluate_expression_internal(right)?;
                            if let Value::Boolean(r) = right_val {
                                Ok(Value::Boolean(r))
                            } else {
                                Err("Runtime error: 'and' operator requires boolean operands.".to_string())
                            }
                        } else {
                            Err("Runtime error: 'and' operator requires boolean operands.".to_string())
                        }
                    }
                    BinaryOperator::Or => {
                        if let Value::Boolean(l) = left_val {
                            if l {
                                return Ok(Value::Boolean(true));
                            }
                            let right_val = self.evaluate_expression_internal(right)?;
                            if let Value::Boolean(r) = right_val {
                                Ok(Value::Boolean(r))
                            } else {
                                Err("Runtime error: 'or' operator requires boolean operands.".to_string())
                            }
                        } else {
                            Err("Runtime error: 'or' operator requires boolean operands.".to_string())
                        }
                    }
                }
            }
            Expression::FunctionCall { name, arguments } => {
                // Handle built-in functions first
                if let Some(result) = self.handle_builtin_function(&name, arguments)? {
                    return Ok(result);
                }
                
                // Get the function definition
                let function = self.env.get_function(&name)
                    .ok_or_else(|| format!(
                        "Runtime error: Undefined function '{}'.\nHint: Check if the function is defined and spelled correctly.",
                        name
                    ))?;
                
                // Evaluate arguments
                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(self.evaluate_expression(arg)?);
                }
                
                // Check argument count
                if arg_values.len() != function.parameters.len() {
                    return Err(format!(
                        "Runtime error: Function '{}' expects {} arguments, got {}.\nHint: Check the number of arguments in the function call.",
                        name,
                        function.parameters.len(),
                        arg_values.len()
                    ));
                }
                
                // Create a new environment for the function
                let mut func_env = Environment::with_parent(self.env.clone());
                
                // Bind arguments to parameters
                for (param, value) in function.parameters.iter().zip(arg_values) {
                    func_env.set(param, value);
                }
                
                // Execute the function body
                let mut interpreter = Interpreter { env: func_env };
                let mut result = Value::Null;
                
                for stmt in &function.body {
                    match interpreter.execute_statement(stmt)? {
                        ExecutionResult::Normal => {},
                        ExecutionResult::Return(value) => {
                            result = value.unwrap_or(Value::Null);
                            break;
                        }
                    }
                }
                
                Ok(result)
            }
            Expression::UnaryOp { operator, operand } => {
                let value = self.evaluate_expression_internal(operand)?;
                match operator {
                    UnaryOperator::Not => {
                        if let Value::Boolean(b) = value {
                            Ok(Value::Boolean(!b))
                        } else {
                            Err("Runtime error: 'not' operator requires a boolean operand.\nHint: Use 'not' only with boolean expressions.".to_string())
                        }
                    }
                }
            }
            Expression::List(items) => {
                let mut evaluated_items = Vec::new();
                for item in items {
                    evaluated_items.push(self.evaluate_expression(item)?);
                }
                Ok(Value::List(evaluated_items))
            }
            Expression::ListIndex { list, index } => {
                let list_value = self.evaluate_expression(&*list)?;
                let index_value = self.evaluate_expression(&*index)?;
                
                if let Value::List(items) = list_value {
                    if let Value::Number(idx) = index_value {
                        if idx < 0 {
                            return Err(format!("Runtime error: List index must be non-negative, got {}.", idx));
                        }
                        let idx_usize = idx as usize;
                        if idx_usize >= items.len() {
                            return Err(format!(
                                "Runtime error: List index {} is out of bounds. List has {} items.",
                                idx_usize, items.len()
                            ));
                        }
                        Ok(items[idx_usize].clone())
                    } else {
                        Err(format!("Runtime error: List index must be a number, got {:?}.", index_value))
                    }
                } else {
                    Err(format!("Runtime error: Cannot index into non-list value {:?}.", list_value))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expression, Statement};
    use crate::parser;
    
    #[test]
    fn test_variable_assignment() {
        let mut interpreter = Interpreter::new();
        
        let stmt = Statement::Assignment {
            variable: "x".to_string(),
            value: Expression::Number(42),
        };
        
        interpreter.execute_statement(&stmt).unwrap();
        
        assert_eq!(
            interpreter.env.get("x"),
            Some(Value::Number(42))
        );
    }
    
    #[test]
    fn test_function_definition_and_call() {
        let mut interpreter = Interpreter::new();
        
        // Define a function that returns its argument
        let func_def = Statement::FunctionDefinition {
            name: "identity".to_string(),
            parameters: vec!["x".to_string()],
            body: vec![
                Statement::Return(Some(Expression::Variable("x".to_string())))
            ],
        };
        
        // Execute function definition
        interpreter.execute_statement(&func_def).unwrap();
        
        // Call the function
        let result = interpreter.evaluate_expression(&Expression::FunctionCall {
            name: "identity".to_string(),
            arguments: vec![Expression::Number(42)],
        }).unwrap();
        
        assert_eq!(result, Value::Number(42));
    }
    
    #[test]
    fn test_addition() {
        let source = "Set x to 5 plus 3. print x.";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("x"), Some(Value::Number(8)));
    }
    
    #[test]
    fn test_subtraction() {
        let source = "Set x to 10 minus 4. print x.";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("x"), Some(Value::Number(6)));
    }
    
    #[test]
    fn test_multiplication() {
        let source = "Set x to 6 times 7. print x.";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("x"), Some(Value::Number(42)));
    }
    
    #[test]
    fn test_division() {
        let source = "Set x to 20 divided by 5. print x.";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("x"), Some(Value::Number(4)));
    }
    
    #[test]
    fn test_complex_expression() {
        let source = "Set x to 2 plus 3 times 4. print x.";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("x"), Some(Value::Number(14)));
    }
    
    #[test]
    fn test_while_loop_basic() {
        let source = "
            Set x to 0.
            While x is less than 3:
                Set x to x + 1.
            print x.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("x"), Some(Value::Number(3)));
    }
    
    #[test]
    fn test_while_loop_false_condition() {
        let source = "
            Set x to 5.
            While x is less than 3:
                Set x to x + 1.
            print x.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // x should remain 5 since condition is false
        assert_eq!(interpreter.env.get("x"), Some(Value::Number(5)));
    }
    
    #[test]
    fn test_while_loop_with_counter() {
        let source = "
            Set counter to 0.
            Set sum to 0.
            While counter is less than 5:
                Set sum to sum + counter.
                Set counter to counter + 1.
            print sum.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        // Sum of 0+1+2+3+4 = 10
        assert_eq!(interpreter.env.get("sum"), Some(Value::Number(10)));
        assert_eq!(interpreter.env.get("counter"), Some(Value::Number(5)));
    }
    
    #[test]
    fn test_list_literal() {
        let source = "Set list to [1, 2, 3]. print list.";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("list"),
            Some(Value::List(vec![
                Value::Number(1),
                Value::Number(2),
                Value::Number(3),
            ]))
        );
    }
    
    #[test]
    fn test_list_indexing() {
        let source = "
            Set list to [10, 20, 30].
            Set first to item 0 of list.
            print first.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("first"), Some(Value::Number(10)));
    }
    
    #[test]
    fn test_list_append() {
        let source = "
            Set list to [1, 2].
            Add 3 to list.
            print list.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(
            interpreter.env.get("list"),
            Some(Value::List(vec![
                Value::Number(1),
                Value::Number(2),
                Value::Number(3),
            ]))
        );
    }
    
    #[test]
    fn test_list_out_of_bounds() {
        let source = "
            Set list to [1, 2].
            Set x to item 5 of list.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of bounds"));
    }
    
    #[test]
    fn test_list_mixed_types() {
        let source = "
            Set list to [1, \"hello\", true].
            Set num to item 0 of list.
            Set str to item 1 of list.
            Set bool to item 2 of list.
        ";
        let program = parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).unwrap();
        assert_eq!(interpreter.env.get("num"), Some(Value::Number(1)));
        assert_eq!(interpreter.env.get("str"), Some(Value::String("hello".to_string())));
        assert_eq!(interpreter.env.get("bool"), Some(Value::Boolean(true)));
    }
} 