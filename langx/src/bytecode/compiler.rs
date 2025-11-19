//! # Bytecode Compiler Module
//!
//! This module provides the compiler that converts LangX AST into bytecode.

use crate::ast::{Program, Statement, Expression, BinaryOperator, UnaryOperator};
use crate::bytecode::chunk::{Chunk, OpCode};
use crate::interpreter::Value;
use std::collections::HashMap;

/// Compiler that converts LangX AST to bytecode.
///
/// The compiler traverses the AST and generates bytecode instructions
/// for each node, maintaining function definitions and handling control flow.
///
/// # Example
///
/// ```rust
/// use langx::{parser, bytecode};
///
/// let source = "Set x to 10.";
/// let program = parser::parse(source)?;
///
/// let mut compiler = bytecode::Compiler::new();
/// let chunk = compiler.compile(&program)?;
/// # Ok::<(), String>(())
/// ```
pub struct Compiler {
    chunk: Chunk,
    functions: HashMap<String, FunctionInfo>,
    loop_starts: Vec<usize>,  // Stack of loop start addresses for break/continue
    loop_ends: Vec<usize>,    // Stack of loop end addresses
}

#[derive(Clone)]
pub struct FunctionInfo {
    pub parameters: Vec<crate::ast::Parameter>,
    pub body: Vec<Statement>,
    pub chunk: Option<Chunk>,  // Compiled function body
}

impl Compiler {
    /// Create a new bytecode compiler.
    ///
    /// # Returns
    ///
    /// A new `Compiler` instance ready to compile AST programs.
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            functions: HashMap::new(),
            loop_starts: Vec::new(),
            loop_ends: Vec::new(),
        }
    }
    
    /// Compile a program to bytecode
    pub fn compile(&mut self, program: &Program) -> Result<Chunk, String> {
        // First pass: collect function definitions
        for statement in &program.statements {
            if let Statement::FunctionDefinition { name, parameters, body } = statement {
                self.functions.insert(
                    name.clone(),
                    FunctionInfo {
                        parameters: parameters.clone(),
                        body: body.clone(),
                        chunk: None,  // Will be compiled in second pass
                    },
                );
            }
        }
        
        // Second pass: compile function bodies
        // We compile functions iteratively, updating the functions map after each compilation
        // so that recursive calls can see previously compiled functions
        
        let function_names: Vec<String> = self.functions.keys().cloned().collect();
        
        for func_name in function_names {
            let func_info = self.functions.get(&func_name).unwrap();
            let body = func_info.body.clone();
            
            // Create compiler with current functions map (which may have some chunks already compiled)
            let mut func_compiler = Compiler {
                chunk: Chunk::new(),
                functions: self.functions.clone(),  // Give access to all functions for recursive calls
                loop_starts: Vec::new(),
                loop_ends: Vec::new(),
            };
            
            for stmt in &body {
                func_compiler.compile_statement(stmt, 0)?;
            }
            // Ensure function ends with return (even if implicit)
            if !body.is_empty() {
                // Check if last statement is a return
                let last_is_return = matches!(body.last(), Some(Statement::Return(_)));
                if !last_is_return {
                    // Add implicit return null
                    let null_idx = func_compiler.chunk.add_constant(Value::Null);
                    func_compiler.chunk.write(OpCode::LoadConstant(null_idx), 0);
                    func_compiler.chunk.write(OpCode::ReturnValue, 0);
                }
            } else {
                // Empty function returns null
                let null_idx = func_compiler.chunk.add_constant(Value::Null);
                func_compiler.chunk.write(OpCode::LoadConstant(null_idx), 0);
                func_compiler.chunk.write(OpCode::ReturnValue, 0);
            }
            
            // Update the function info with the compiled chunk
            if let Some(func_info_mut) = self.functions.get_mut(&func_name) {
                func_info_mut.chunk = Some(func_compiler.chunk.clone());
            }
        }
        
        // Third pass: compile main program statements
        for statement in &program.statements {
            match statement {
                Statement::FunctionDefinition { .. } => {
                    // Function definitions are already compiled
                }
                _ => {
                    self.compile_statement(statement, 0)?;
                }
            }
        }
        
        Ok(self.chunk.clone())
    }
    
    fn compile_statement(&mut self, statement: &Statement, line: usize) -> Result<(), String> {
        match statement {
            Statement::Assignment { variable, value } => {
                self.compile_expression(value, line)?;
                self.chunk.write(OpCode::StoreVariable(variable.clone()), line);
                Ok(())
            }
            
            Statement::Conditional { condition, then_branch } => {
                self.compile_expression(condition, line)?;
                let jump_if_false_addr = self.chunk.write(OpCode::JumpIfFalse(0), line);
                self.compile_statement(then_branch, line)?;
                let after_then_addr = self.chunk.len();
                self.chunk.patch_jump(jump_if_false_addr, after_then_addr);
                Ok(())
            }
            
            Statement::Print(expr) => {
                self.compile_expression(expr, line)?;
                self.chunk.write(OpCode::Print, line);
                Ok(())
            }
            
            Statement::Repeat { count, body } => {
                self.compile_expression(count, line)?;
                // Store count in a temporary variable
                let temp_var = format!("__repeat_count_{}", self.chunk.len());
                self.chunk.write(OpCode::StoreVariable(temp_var.clone()), line);
                
                // Load 0 as counter
                let counter_var = format!("__repeat_i_{}", self.chunk.len());
                let zero_idx = self.chunk.add_constant(Value::Number(0));
                self.chunk.write(OpCode::LoadConstant(zero_idx), line);
                self.chunk.write(OpCode::StoreVariable(counter_var.clone()), line);
                
                // Loop start
                let loop_start = self.chunk.len();
                self.loop_starts.push(loop_start);
                
                // Check if counter < count
                self.chunk.write(OpCode::LoadVariable(counter_var.clone()), line);
                self.chunk.write(OpCode::LoadVariable(temp_var.clone()), line);
                self.chunk.write(OpCode::LessThan, line);  // Compare: counter < count
                let jump_if_false_addr = self.chunk.write(OpCode::JumpIfFalse(0), line);
                self.loop_ends.push(jump_if_false_addr);
                
                // Body
                self.compile_statement(body, line)?;
                
                // Increment counter
                let one_idx = self.chunk.add_constant(Value::Number(1));
                self.chunk.write(OpCode::LoadVariable(counter_var.clone()), line);
                self.chunk.write(OpCode::LoadConstant(one_idx), line);
                self.chunk.write(OpCode::Add, line);
                self.chunk.write(OpCode::StoreVariable(counter_var.clone()), line);
                
                // Jump back to loop start
                self.chunk.write(OpCode::JumpBackward(loop_start), line);
                
                // Patch jump
                let loop_end = self.chunk.len();
                self.chunk.patch_jump(jump_if_false_addr, loop_end);
                self.loop_ends.pop();
                self.loop_starts.pop();
                
                Ok(())
            }
            
            Statement::While { condition, body } => {
                let loop_start = self.chunk.len();
                self.loop_starts.push(loop_start);
                
                self.compile_expression(condition, line)?;
                let jump_if_false_addr = self.chunk.write(OpCode::JumpIfFalse(0), line);
                self.loop_ends.push(jump_if_false_addr);
                
                self.compile_statement(body, line)?;
                
                // Jump back to condition
                self.chunk.write(OpCode::JumpBackward(loop_start), line);
                
                // Patch jump
                let loop_end = self.chunk.len();
                self.chunk.patch_jump(jump_if_false_addr, loop_end);
                self.loop_ends.pop();
                self.loop_starts.pop();
                
                Ok(())
            }
            
            Statement::For { variable, list, body } => {
                // Evaluate list expression
                self.compile_expression(list, line)?;
                // Store list in temporary variable
                let list_var = format!("__for_list_{}", self.chunk.len());
                self.chunk.write(OpCode::StoreVariable(list_var.clone()), line);
                
                // Initialize index to 0
                let index_var = format!("__for_index_{}", self.chunk.len());
                let zero_idx = self.chunk.add_constant(Value::Number(0));
                self.chunk.write(OpCode::LoadConstant(zero_idx), line);
                self.chunk.write(OpCode::StoreVariable(index_var.clone()), line);
                
                // Loop start
                let loop_start = self.chunk.len();
                self.loop_starts.push(loop_start);
                
                // Check if index < list length
                self.chunk.write(OpCode::LoadVariable(index_var.clone()), line);
                self.chunk.write(OpCode::LoadVariable(list_var.clone()), line);
                // Get list length (builtin call)
                self.chunk.write(OpCode::CallBuiltin("list_length".to_string(), 1), line);
                let jump_if_false_addr = self.chunk.write(OpCode::JumpIfFalse(0), line);
                self.loop_ends.push(jump_if_false_addr);
                
                // Load current item: list[index]
                self.chunk.write(OpCode::LoadVariable(list_var.clone()), line);
                self.chunk.write(OpCode::LoadVariable(index_var.clone()), line);
                self.chunk.write(OpCode::ListIndex, line);
                // Store in loop variable
                self.chunk.write(OpCode::StoreVariable(variable.clone()), line);
                
                // Body
                self.compile_statement(body, line)?;
                
                // Increment index
                let one_idx = self.chunk.add_constant(Value::Number(1));
                self.chunk.write(OpCode::LoadVariable(index_var.clone()), line);
                self.chunk.write(OpCode::LoadConstant(one_idx), line);
                self.chunk.write(OpCode::Add, line);
                self.chunk.write(OpCode::StoreVariable(index_var.clone()), line);
                
                // Jump back to loop start
                self.chunk.write(OpCode::JumpBackward(loop_start), line);
                
                // Patch jump
                let loop_end = self.chunk.len();
                self.chunk.patch_jump(jump_if_false_addr, loop_end);
                self.loop_ends.pop();
                self.loop_starts.pop();
                
                Ok(())
            }
            
            Statement::Block(statements) => {
                for stmt in statements {
                    self.compile_statement(stmt, line)?;
                }
                Ok(())
            }
            
            Statement::FunctionDefinition { .. } => {
                // Function definitions are collected in first pass
                Ok(())
            }
            
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    self.compile_expression(expr, line)?;
                    self.chunk.write(OpCode::ReturnValue, line);
                } else {
                    self.chunk.write(OpCode::Return, line);
                }
                Ok(())
            }
            
            Statement::ListAppend { list_name, value } => {
                self.compile_expression(value, line)?;
                self.chunk.write(OpCode::ListAppend(list_name.clone()), line);
                Ok(())
            }
            
            Statement::MapAssignment { map_name, key, value } => {
                self.compile_expression(key, line)?;
                self.compile_expression(value, line)?;
                self.chunk.write(OpCode::MapStore(map_name.clone()), line);
                Ok(())
            }
            
            Statement::Break => {
                if let Some(&loop_end) = self.loop_ends.last() {
                    self.chunk.write(OpCode::Jump(loop_end), line);
                } else {
                    return Err("Break statement outside of loop.".to_string());
                }
                Ok(())
            }
            
            Statement::Continue => {
                if let Some(&loop_start) = self.loop_starts.last() {
                    self.chunk.write(OpCode::JumpBackward(loop_start), line);
                } else {
                    return Err("Continue statement outside of loop.".to_string());
                }
                Ok(())
            }
        }
    }
    
    fn compile_expression(&mut self, expr: &Expression, line: usize) -> Result<(), String> {
        match expr {
            Expression::Number(n) => {
                let idx = self.chunk.add_constant(Value::Number(*n));
                self.chunk.write(OpCode::LoadConstant(idx), line);
                Ok(())
            }
            
            Expression::String(s) => {
                let idx = self.chunk.add_constant(Value::String(s.clone()));
                self.chunk.write(OpCode::LoadConstant(idx), line);
                Ok(())
            }
            
            Expression::Boolean(b) => {
                let idx = self.chunk.add_constant(Value::Boolean(*b));
                self.chunk.write(OpCode::LoadConstant(idx), line);
                Ok(())
            }
            
            Expression::Variable(name) => {
                self.chunk.write(OpCode::LoadVariable(name.clone()), line);
                Ok(())
            }
            
            Expression::BinaryOp { left, operator, right } => {
                self.compile_expression(left, line)?;
                self.compile_expression(right, line)?;
                
                match operator {
                    BinaryOperator::Plus => {
                        self.chunk.write(OpCode::Add, line);
                    }
                    BinaryOperator::Minus => {
                        self.chunk.write(OpCode::Subtract, line);
                    }
                    BinaryOperator::Times => {
                        self.chunk.write(OpCode::Multiply, line);
                    }
                    BinaryOperator::Divide => {
                        self.chunk.write(OpCode::Divide, line);
                    }
                    BinaryOperator::GreaterThan => {
                        self.chunk.write(OpCode::GreaterThan, line);
                    }
                    BinaryOperator::LessThan => {
                        self.chunk.write(OpCode::LessThan, line);
                    }
                    BinaryOperator::Equal => {
                        self.chunk.write(OpCode::Equal, line);
                    }
                    BinaryOperator::NotEqual => {
                        self.chunk.write(OpCode::NotEqual, line);
                    }
                    BinaryOperator::And => {
                        self.chunk.write(OpCode::And, line);
                    }
                    BinaryOperator::Or => {
                        self.chunk.write(OpCode::Or, line);
                    }
                }
                Ok(())
            }
            
            Expression::UnaryOp { operator, operand } => {
                self.compile_expression(operand, line)?;
                match operator {
                    UnaryOperator::Not => {
                        self.chunk.write(OpCode::Not, line);
                    }
                }
                Ok(())
            }
            
            Expression::FunctionCall { name, arguments } => {
                // Check if it's a builtin function
                let builtin_functions = [
                    "string_length", "substring", "split", "join", "replace",
                    "abs", "min", "max", "pow", "sqrt", "round", "floor", "ceil",
                    "read_file", "write_file",
                    "current_timestamp", "current_datetime", "format_timestamp", "time_difference",
                ];
                
                if builtin_functions.contains(&name.as_str()) {
                    // Compile arguments
                    for arg in arguments {
                        self.compile_expression(arg, line)?;
                    }
                    self.chunk.write(OpCode::CallBuiltin(name.clone(), arguments.len()), line);
                } else {
                    // Regular function call
                    // Check if function exists
                    if !self.functions.contains_key(name) {
                        return Err(format!("Undefined function '{}'", name));
                    }
                    
                    // Compile arguments
                    for arg in arguments {
                        self.compile_expression(arg, line)?;
                    }
                    self.chunk.write(OpCode::CallFunction(name.clone(), arguments.len()), line);
                }
                Ok(())
            }
            
            Expression::List(items) => {
                // Compile all items
                for item in items {
                    self.compile_expression(item, line)?;
                }
                self.chunk.write(OpCode::BuildList(items.len()), line);
                Ok(())
            }
            
            Expression::ListIndex { list, index } => {
                self.compile_expression(list, line)?;
                self.compile_expression(index, line)?;
                self.chunk.write(OpCode::ListIndex, line);
                Ok(())
            }
            
            Expression::Map(entries) => {
                // Compile key-value pairs (key, value, key, value, ...)
                for (key, value) in entries {
                    self.compile_expression(key, line)?;
                    self.compile_expression(value, line)?;
                }
                self.chunk.write(OpCode::BuildMap(entries.len()), line);
                Ok(())
            }
            
            Expression::MapIndex { map, key } => {
                self.compile_expression(map, line)?;
                self.compile_expression(key, line)?;
                self.chunk.write(OpCode::MapIndex, line);
                Ok(())
            }
        }
    }
    
    /// Get function definitions (for VM to use)
    pub fn get_functions(&self) -> &HashMap<String, FunctionInfo> {
        &self.functions
    }
    
    /// Get mutable reference to functions (for taking ownership)
    pub fn get_functions_mut(&mut self) -> &mut HashMap<String, FunctionInfo> {
        &mut self.functions
    }
}

