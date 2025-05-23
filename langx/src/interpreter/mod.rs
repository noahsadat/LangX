use crate::ast::{Program, Statement, Expression, BinaryOperator, UnaryOperator};
use std::collections::HashMap;

/// Value types that can be stored in variables
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
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
        Self {
            env: Environment::new(),
        }
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
        }
    }
    
    /// Take a left-associative tree of +,−,×,÷ and re-group it so ×,÷ bind tighter than +,−.
    fn apply_precedence(&self, expr: Expression) -> Expression {
        match expr {
            Expression::BinaryOp { left, operator, right } => {
                // Recursively fix children
                let left = Box::new(self.apply_precedence(*left));
                let right = Box::new(self.apply_precedence(*right));
                // If this is a +/−, we just rebuild
                if matches!(operator, BinaryOperator::Plus | BinaryOperator::Minus) {
                    Expression::BinaryOp { left, operator, right }
                }
                // If this is a ×/÷, we need to pull up any +/− on the left
                else {
                    if let Expression::BinaryOp { left: ref ll, operator: ref lop, right: ref lr } = *left {
                        if matches!(lop, BinaryOperator::Plus | BinaryOperator::Minus) {
                            // a + (b × c)
                            let new_right = Box::new(Expression::BinaryOp {
                                left: lr.clone(),
                                operator: operator.clone(),
                                right,
                            });
                            return Expression::BinaryOp {
                                left: ll.clone(),
                                operator: lop.clone(),
                                right: Box::new(self.apply_precedence(*new_right)),
                            };
                        }
                    }
                    Expression::BinaryOp { left, operator, right }
                }
            }
            other => other,
        }
    }
    
    fn evaluate_expression(&self, expr: &Expression) -> Result<Value, String> {
        // Before we do anything, regroup the tree for correct precedence:
        let expr = self.apply_precedence(expr.clone());
        match expr {
            Expression::Number(n) => Ok(Value::Number(n)),
            Expression::String(s) => Ok(Value::String(s)),
            Expression::Variable(name) => {
                self.env.get(&name)
                    .ok_or_else(|| format!(
                        "Runtime error: Undefined variable '{}'.\nHint: Check if the variable is declared and spelled correctly.",
                        name
                    ))
            }
            Expression::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expression(&left)?;
                let right_val = self.evaluate_expression(&right)?;
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
                            _ => Err(format!("Cannot add {:?} and {:?}", left_val, right_val))
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
                            let right_val = self.evaluate_expression(&right)?;
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
                            let right_val = self.evaluate_expression(&right)?;
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
                // Get the function definition
                let function = self.env.get_function(&name)
                    .ok_or_else(|| format!(
                        "Runtime error: Undefined function '{}'.\nHint: Check if the function is defined and spelled correctly.",
                        name
                    ))?;
                
                // Evaluate arguments
                let mut arg_values = Vec::new();
                for arg in &arguments {
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
                let value = self.evaluate_expression(operand)?;
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
} 