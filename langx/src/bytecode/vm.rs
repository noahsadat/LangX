use crate::bytecode::chunk::{Chunk, OpCode};
use crate::bytecode::compiler::FunctionInfo;
use crate::interpreter::{Value, Environment};
use std::collections::HashMap;

/// Stack-based Virtual Machine for executing bytecode
pub struct VM {
    stack: Vec<Value>,
    environment: Environment,
    functions: HashMap<String, FunctionInfo>,
    ip: usize,  // Instruction pointer
    chunk: Option<Chunk>,
    call_stack: Vec<CallFrame>,
}

struct CallFrame {
    return_address: usize,
    local_env: Environment,
    local_chunk: Option<Chunk>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            environment: Environment::new(),
            functions: HashMap::new(),
            ip: 0,
            chunk: None,
            call_stack: Vec::new(),
        }
    }
    
    /// Execute a chunk of bytecode
    pub fn execute(&mut self, chunk: Chunk, functions: HashMap<String, FunctionInfo>) -> Result<(), String> {
        self.chunk = Some(chunk);
        self.functions = functions;
        self.ip = 0;
        
        while self.ip < self.chunk.as_ref().unwrap().code.len() {
            let op = self.chunk.as_ref().unwrap().code[self.ip].clone();
            self.ip += 1;
            
            match self.execute_instruction(&op)? {
                ControlFlow::Continue => continue,
                ControlFlow::Return => break,
                ControlFlow::Jump(addr) => {
                    self.ip = addr;
                    continue;
                }
            }
        }
        
        Ok(())
    }
    
    fn execute_instruction(&mut self, op: &OpCode) -> Result<ControlFlow, String> {
        let chunk = self.chunk.as_ref().unwrap();
        
        match op {
            OpCode::LoadConstant(idx) => {
                let value = chunk.constants[*idx].clone();
                self.stack.push(value);
                Ok(ControlFlow::Continue)
            }
            
            OpCode::LoadVariable(name) => {
                let value = self.environment.get(name)
                    .ok_or_else(|| format!("Undefined variable '{}'", name))?;
                self.stack.push(value);
                Ok(ControlFlow::Continue)
            }
            
            OpCode::StoreVariable(name) => {
                let value = self.stack.pop()
                    .ok_or_else(|| "Stack underflow".to_string())?;
                self.environment.set(name, value);
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Add => {
                let right = self.pop()?;
                let left = self.pop()?;
                let result = self.add_values(left, right)?;
                self.stack.push(result);
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Subtract => {
                let right = self.pop()?;
                let left = self.pop()?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        self.stack.push(Value::Number(l - r));
                    }
                    _ => return Err("Cannot subtract non-numbers".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Multiply => {
                let right = self.pop()?;
                let left = self.pop()?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        self.stack.push(Value::Number(l * r));
                    }
                    _ => return Err("Cannot multiply non-numbers".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Divide => {
                let right = self.pop()?;
                let left = self.pop()?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        if r == 0 {
                            return Err("Division by zero".to_string());
                        }
                        self.stack.push(Value::Number(l / r));
                    }
                    _ => return Err("Cannot divide non-numbers".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::GreaterThan => {
                let right = self.pop()?;
                let left = self.pop()?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        self.stack.push(Value::Boolean(l > r));
                    }
                    _ => return Err("Cannot compare non-numbers".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::LessThan => {
                let right = self.pop()?;
                let left = self.pop()?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        self.stack.push(Value::Boolean(l < r));
                    }
                    _ => return Err("Cannot compare non-numbers".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Equal => {
                let right = self.pop()?;
                let left = self.pop()?;
                self.stack.push(Value::Boolean(left == right));
                Ok(ControlFlow::Continue)
            }
            
            OpCode::NotEqual => {
                let right = self.pop()?;
                let left = self.pop()?;
                self.stack.push(Value::Boolean(left != right));
                Ok(ControlFlow::Continue)
            }
            
            OpCode::And => {
                let right = self.pop()?;
                let left = self.pop()?;
                match (left, right) {
                    (Value::Boolean(l), Value::Boolean(r)) => {
                        self.stack.push(Value::Boolean(l && r));
                    }
                    _ => return Err("Cannot AND non-booleans".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Or => {
                let right = self.pop()?;
                let left = self.pop()?;
                match (left, right) {
                    (Value::Boolean(l), Value::Boolean(r)) => {
                        self.stack.push(Value::Boolean(l || r));
                    }
                    _ => return Err("Cannot OR non-booleans".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Not => {
                let val = self.pop()?;
                match val {
                    Value::Boolean(b) => {
                        self.stack.push(Value::Boolean(!b));
                    }
                    _ => return Err("Cannot NOT non-boolean".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Jump(addr) => {
                Ok(ControlFlow::Jump(*addr))
            }
            
            OpCode::JumpIfFalse(addr) => {
                let condition = self.pop()?;
                match condition {
                    Value::Boolean(false) => Ok(ControlFlow::Jump(*addr)),
                    _ => Ok(ControlFlow::Continue),
                }
            }
            
            OpCode::JumpIfTrue(addr) => {
                let condition = self.pop()?;
                match condition {
                    Value::Boolean(true) => Ok(ControlFlow::Jump(*addr)),
                    _ => Ok(ControlFlow::Continue),
                }
            }
            
            OpCode::JumpBackward(addr) => {
                Ok(ControlFlow::Jump(*addr))
            }
            
            OpCode::CallFunction(name, argc) => {
                // Collect arguments from stack first
                let mut args = Vec::new();
                for _ in 0..*argc {
                    args.insert(0, self.pop()?);
                }
                
                // Get function info after popping args
                let func_info = self.functions.get(name)
                    .ok_or_else(|| format!("Undefined function '{}'", name))?;
                
                // Get the compiled function chunk
                let func_chunk = func_info.chunk.as_ref()
                    .ok_or_else(|| format!("Function '{}' has no compiled body", name))?;
                
                // Handle variadic and default parameters
                let mut func_env = Environment::with_parent(self.environment.clone());
                let mut arg_iter = args.into_iter();
                let mut variadic_args = Vec::new();
                
                for param in &func_info.parameters {
                    if param.is_variadic {
                        // Collect remaining arguments
                        while let Some(arg) = arg_iter.next() {
                            variadic_args.push(arg);
                        }
                        // Store variadic args as a list
                        func_env.set(&param.name, Value::List(variadic_args.clone()));
                    } else {
                        if let Some(arg) = arg_iter.next() {
                            func_env.set(&param.name, arg);
                        } else if param.default_value.is_some() {
                            // Evaluate default value (simplified - would need AST interpreter)
                            // For now, just use Null as placeholder
                            func_env.set(&param.name, Value::Null);
                        } else {
                            return Err(format!("Function '{}' expects {} arguments, got {}", name, func_info.parameters.len(), argc));
                        }
                    }
                }
                
                // Save current state
                let return_addr = self.ip;
                let old_chunk = self.chunk.take();
                let old_env = std::mem::replace(&mut self.environment, func_env);
                self.call_stack.push(CallFrame {
                    return_address: return_addr,
                    local_env: old_env,
                    local_chunk: old_chunk,
                });
                
                // Switch to function chunk and execute
                self.chunk = Some(func_chunk.clone());
                self.ip = 0;
                
                // Execute function body
                while self.ip < self.chunk.as_ref().unwrap().code.len() {
                    let op = self.chunk.as_ref().unwrap().code[self.ip].clone();
                    self.ip += 1;
                    
                    match self.execute_instruction(&op)? {
                        ControlFlow::Continue => continue,
                        ControlFlow::Return => {
                            // Function returned without value
                            break;
                        }
                        ControlFlow::Jump(addr) => {
                            self.ip = addr;
                            continue;
                        }
                    }
                }
                
                // Restore previous state
                let return_value = self.stack.pop();  // May be None if Return (not ReturnValue)
                if let Some(frame) = self.call_stack.pop() {
                    self.environment = frame.local_env;
                    self.chunk = frame.local_chunk;
                    self.ip = frame.return_address;
                    // Push return value back if it exists
                    if let Some(val) = return_value {
                        self.stack.push(val);
                    }
                }
                
                Ok(ControlFlow::Continue)
            }
            
            OpCode::CallBuiltin(name, argc) => {
                let mut args = Vec::new();
                for _ in 0..*argc {
                    args.insert(0, self.pop()?);
                }
                
                let result = self.call_builtin(name, &args)?;
                if let Some(val) = result {
                    self.stack.push(val);
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Return => {
                if let Some(frame) = self.call_stack.pop() {
                    self.environment = frame.local_env;
                    self.chunk = frame.local_chunk;
                    self.ip = frame.return_address;
                    Ok(ControlFlow::Continue)
                } else {
                    Ok(ControlFlow::Return)
                }
            }
            
            OpCode::ReturnValue => {
                let value = self.pop()?;
                if let Some(frame) = self.call_stack.pop() {
                    self.environment = frame.local_env;
                    self.chunk = frame.local_chunk;
                    self.ip = frame.return_address;
                    self.stack.push(value);
                    Ok(ControlFlow::Continue)
                } else {
                    self.stack.push(value);
                    Ok(ControlFlow::Return)
                }
            }
            
            OpCode::BuildList(count) => {
                let mut items = Vec::new();
                for _ in 0..*count {
                    items.insert(0, self.pop()?);
                }
                self.stack.push(Value::List(items));
                Ok(ControlFlow::Continue)
            }
            
            OpCode::BuildMap(count) => {
                let mut map = HashMap::new();
                for _ in 0..*count {
                    let value = self.pop()?;
                    let key = self.pop()?;
                    let key_str = match key {
                        Value::String(s) => s,
                        Value::Number(n) => n.to_string(),
                        Value::Boolean(b) => b.to_string(),
                        _ => return Err("Map keys must be string, number, or boolean".to_string()),
                    };
                    map.insert(key_str, value);
                }
                self.stack.push(Value::Map(map));
                Ok(ControlFlow::Continue)
            }
            
            OpCode::ListIndex => {
                let index = self.pop()?;
                let list = self.pop()?;
                match (list, index) {
                    (Value::List(items), Value::Number(idx)) => {
                        if idx < 0 || idx as usize >= items.len() {
                            return Err(format!("List index {} out of bounds", idx));
                        }
                        self.stack.push(items[idx as usize].clone());
                    }
                    _ => return Err("Cannot index non-list or non-number index".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::MapIndex => {
                let key = self.pop()?;
                let map = self.pop()?;
                match (map, key) {
                    (Value::Map(m), Value::String(k)) => {
                        let value = m.get(&k)
                            .cloned()
                            .unwrap_or(Value::Null);
                        self.stack.push(value);
                    }
                    (Value::Map(m), Value::Number(n)) => {
                        let k = n.to_string();
                        let value = m.get(&k)
                            .cloned()
                            .unwrap_or(Value::Null);
                        self.stack.push(value);
                    }
                    (Value::Map(m), Value::Boolean(b)) => {
                        let k = b.to_string();
                        let value = m.get(&k)
                            .cloned()
                            .unwrap_or(Value::Null);
                        self.stack.push(value);
                    }
                    _ => return Err("Cannot index non-map or invalid key type".to_string()),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::ListAppend(name) => {
                let value = self.pop()?;
                let list = self.environment.get(name)
                    .ok_or_else(|| format!("Undefined variable '{}'", name))?;
                match list {
                    Value::List(mut items) => {
                        items.push(value);
                        self.environment.set(name, Value::List(items));
                    }
                    _ => return Err(format!("Cannot append to non-list variable '{}'", name)),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::MapStore(name) => {
                let value = self.pop()?;
                let key = self.pop()?;
                let map = self.environment.get(name)
                    .ok_or_else(|| format!("Undefined variable '{}'", name))?;
                match map {
                    Value::Map(mut m) => {
                        let key_str = match key {
                            Value::String(s) => s,
                            Value::Number(n) => n.to_string(),
                            Value::Boolean(b) => b.to_string(),
                            _ => return Err("Map keys must be string, number, or boolean".to_string()),
                        };
                        m.insert(key_str, value);
                        self.environment.set(name, Value::Map(m));
                    }
                    _ => return Err(format!("Cannot store in non-map variable '{}'", name)),
                }
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Print => {
                let value = self.pop()?;
                println!("{}", value);
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Pop => {
                self.pop()?;
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Dup => {
                let value = self.stack.last()
                    .ok_or_else(|| "Stack underflow".to_string())?
                    .clone();
                self.stack.push(value);
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Break => {
                // Break is handled by jumps in the bytecode
                Ok(ControlFlow::Continue)
            }
            
            OpCode::Continue => {
                // Continue is handled by jumps in the bytecode
                Ok(ControlFlow::Continue)
            }
            
            OpCode::LoadNull => {
                self.stack.push(Value::Null);
                Ok(ControlFlow::Continue)
            }
        }
    }
    
    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow".to_string())
    }
    
    fn add_values(&self, left: Value, right: Value) -> Result<Value, String> {
        match (&left, &right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::String(_), _) | (_, Value::String(_)) => {
                Ok(Value::String(format!("{}{}", left, right)))
            }
            _ => Err(format!("Cannot add {:?} and {:?}", left, right)),
        }
    }
    
    fn call_builtin(&self, name: &str, args: &[Value]) -> Result<Option<Value>, String> {
        match name {
            "string_length" => {
                if args.len() != 1 {
                    return Err(format!("Built-in function 'string_length' expects 1 argument, got {}.", args.len()));
                }
                if let Value::String(s) = &args[0] {
                    Ok(Some(Value::Number(s.len() as i64)))
                } else {
                    Err(format!("Built-in function 'string_length' expects a string argument, got {:?}.", args[0]))
                }
            }
            "substring" => {
                if args.len() != 3 {
                    return Err(format!("Built-in function 'substring' expects 3 arguments, got {}.", args.len()));
                }
                if let (Value::String(s), Value::Number(start), Value::Number(len)) = (&args[0], &args[1], &args[2]) {
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
                    Err(format!("Built-in function 'substring' expects (string, number, number) arguments."))
                }
            }
            "split" => {
                if args.len() != 2 {
                    return Err(format!("Built-in function 'split' expects 2 arguments, got {}.", args.len()));
                }
                if let (Value::String(s), Value::String(delimiter)) = (&args[0], &args[1]) {
                    let parts: Vec<Value> = s.split(delimiter)
                        .map(|part| Value::String(part.to_string()))
                        .collect();
                    Ok(Some(Value::List(parts)))
                } else {
                    Err(format!("Built-in function 'split' expects (string, string) arguments."))
                }
            }
            "join" => {
                if args.len() != 2 {
                    return Err(format!("Built-in function 'join' expects 2 arguments, got {}.", args.len()));
                }
                if let (Value::List(items), Value::String(delimiter)) = (&args[0], &args[1]) {
                    let mut parts = Vec::new();
                    for item in items {
                        match item {
                            Value::String(s) => parts.push(s.clone()),
                            Value::Number(n) => parts.push(n.to_string()),
                            Value::Boolean(b) => parts.push(b.to_string()),
                            Value::Null => parts.push("null".to_string()),
                            _ => return Err("Built-in function 'join' cannot join nested structures.".to_string()),
                        }
                    }
                    Ok(Some(Value::String(parts.join(delimiter))))
                } else {
                    Err(format!("Built-in function 'join' expects (list, string) arguments."))
                }
            }
            "replace" => {
                if args.len() != 3 {
                    return Err(format!("Built-in function 'replace' expects 3 arguments, got {}.", args.len()));
                }
                if let (Value::String(s), Value::String(old), Value::String(new)) = (&args[0], &args[1], &args[2]) {
                    Ok(Some(Value::String(s.replace(old, new))))
                } else {
                    Err(format!("Built-in function 'replace' expects (string, string, string) arguments."))
                }
            }
            "abs" => {
                if args.len() != 1 {
                    return Err(format!("Built-in function 'abs' expects 1 argument, got {}.", args.len()));
                }
                if let Value::Number(n) = &args[0] {
                    Ok(Some(Value::Number(n.abs())))
                } else {
                    Err(format!("Built-in function 'abs' expects a number argument."))
                }
            }
            "min" => {
                if args.len() != 2 {
                    return Err(format!("Built-in function 'min' expects 2 arguments, got {}.", args.len()));
                }
                if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
                    Ok(Some(Value::Number(*a.min(b))))
                } else {
                    Err(format!("Built-in function 'min' expects number arguments."))
                }
            }
            "max" => {
                if args.len() != 2 {
                    return Err(format!("Built-in function 'max' expects 2 arguments, got {}.", args.len()));
                }
                if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
                    Ok(Some(Value::Number(*a.max(b))))
                } else {
                    Err(format!("Built-in function 'max' expects number arguments."))
                }
            }
            "pow" => {
                if args.len() != 2 {
                    return Err(format!("Built-in function 'pow' expects 2 arguments, got {}.", args.len()));
                }
                if let (Value::Number(base), Value::Number(exp)) = (&args[0], &args[1]) {
                    Ok(Some(Value::Number(base.pow(*exp as u32))))
                } else {
                    Err(format!("Built-in function 'pow' expects number arguments."))
                }
            }
            "sqrt" => {
                if args.len() != 1 {
                    return Err(format!("Built-in function 'sqrt' expects 1 argument, got {}.", args.len()));
                }
                if let Value::Number(n) = &args[0] {
                    if *n < 0 {
                        return Err("Built-in function 'sqrt' requires non-negative argument.".to_string());
                    }
                    Ok(Some(Value::Number((*n as f64).sqrt() as i64)))
                } else {
                    Err(format!("Built-in function 'sqrt' expects a number argument."))
                }
            }
            "round" => {
                if args.len() != 1 {
                    return Err(format!("Built-in function 'round' expects 1 argument, got {}.", args.len()));
                }
                if let Value::Number(n) = &args[0] {
                    Ok(Some(Value::Number((*n as f64).round() as i64)))
                } else {
                    Err(format!("Built-in function 'round' expects a number argument."))
                }
            }
            "floor" => {
                if args.len() != 1 {
                    return Err(format!("Built-in function 'floor' expects 1 argument, got {}.", args.len()));
                }
                if let Value::Number(n) = &args[0] {
                    Ok(Some(Value::Number((*n as f64).floor() as i64)))
                } else {
                    Err(format!("Built-in function 'floor' expects a number argument."))
                }
            }
            "ceil" => {
                if args.len() != 1 {
                    return Err(format!("Built-in function 'ceil' expects 1 argument, got {}.", args.len()));
                }
                if let Value::Number(n) = &args[0] {
                    Ok(Some(Value::Number((*n as f64).ceil() as i64)))
                } else {
                    Err(format!("Built-in function 'ceil' expects a number argument."))
                }
            }
            "list_length" => {
                if args.len() != 1 {
                    return Err(format!("Built-in function 'list_length' expects 1 argument, got {}.", args.len()));
                }
                if let Value::List(list) = &args[0] {
                    Ok(Some(Value::Number(list.len() as i64)))
                } else {
                    Err(format!("Built-in function 'list_length' expects a list argument."))
                }
            }
            _ => Err(format!("Unknown built-in function '{}'", name)),
        }
    }
}

enum ControlFlow {
    Continue,
    Return,
    Jump(usize),
}

