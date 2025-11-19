use crate::interpreter::Value;
use std::fmt;

/// Bytecode instruction opcodes
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // Constants
    LoadConstant(usize),  // Load constant from constant pool
    
    // Variables
    LoadVariable(String),  // Load variable onto stack
    StoreVariable(String), // Store top of stack to variable
    
    // Arithmetic operations
    Add,      // Pop two values, add them, push result
    Subtract, // Pop two values, subtract them, push result
    Multiply, // Pop two values, multiply them, push result
    Divide,   // Pop two values, divide them, push result
    
    // Comparison operations
    GreaterThan,  // Pop two values, compare, push boolean
    LessThan,     // Pop two values, compare, push boolean
    Equal,        // Pop two values, compare equality, push boolean
    NotEqual,     // Pop two values, compare inequality, push boolean
    
    // Logical operations
    And,  // Pop two booleans, AND them, push result
    Or,   // Pop two booleans, OR them, push result
    Not,  // Pop one boolean, negate it, push result
    
    // Control flow
    Jump(usize),           // Unconditional jump to address
    JumpIfFalse(usize),    // Pop boolean, jump if false
    JumpIfTrue(usize),     // Pop boolean, jump if true
    JumpBackward(usize),   // Jump backward (for loops)
    
    // Functions
    CallFunction(String, usize), // Call function with name and arg count
    CallBuiltin(String, usize),  // Call builtin function with name and arg count
    Return,                      // Return from function
    ReturnValue,                 // Return with value from stack
    
    // Data structures
    BuildList(usize),      // Build list from N items on stack
    BuildMap(usize),       // Build map from N key-value pairs on stack
    ListIndex,             // Pop index and list, push list[index]
    MapIndex,              // Pop key and map, push map[key]
    ListAppend(String),    // Append value to list variable
    MapStore(String),      // Store value in map at key
    
    // Other operations
    Print,                 // Pop value and print it
    Pop,                   // Pop and discard top of stack
    Dup,                   // Duplicate top of stack
    
    // Loop control
    Break,                 // Break out of loop
    Continue,              // Continue to next iteration
    
    // Null/None
    LoadNull,             // Push null value
}

/// A chunk of bytecode with constants and instructions
#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub line_numbers: Vec<usize>, // Line number for each instruction (for debugging)
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            line_numbers: Vec::new(),
        }
    }
    
    /// Add an instruction to the chunk
    pub fn write(&mut self, op: OpCode, line: usize) -> usize {
        let address = self.code.len();
        self.code.push(op);
        self.line_numbers.push(line);
        address
    }
    
    /// Add a constant to the constant pool and return its index
    pub fn add_constant(&mut self, value: Value) -> usize {
        // Check if constant already exists
        for (i, existing) in self.constants.iter().enumerate() {
            if existing == &value {
                return i;
            }
        }
        let index = self.constants.len();
        self.constants.push(value);
        index
    }
    
    /// Get the current instruction count
    pub fn len(&self) -> usize {
        self.code.len()
    }
    
    /// Patch a jump address at the given instruction index
    pub fn patch_jump(&mut self, address: usize, target: usize) {
        match &self.code[address] {
            OpCode::Jump(_) => {
                self.code[address] = OpCode::Jump(target);
            }
            OpCode::JumpIfFalse(_) => {
                self.code[address] = OpCode::JumpIfFalse(target);
            }
            OpCode::JumpIfTrue(_) => {
                self.code[address] = OpCode::JumpIfTrue(target);
            }
            OpCode::JumpBackward(_) => {
                self.code[address] = OpCode::JumpBackward(target);
            }
            _ => panic!("Cannot patch non-jump instruction"),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::LoadConstant(idx) => write!(f, "LOAD_CONST {}", idx),
            OpCode::LoadVariable(name) => write!(f, "LOAD_VAR {}", name),
            OpCode::StoreVariable(name) => write!(f, "STORE_VAR {}", name),
            OpCode::Add => write!(f, "ADD"),
            OpCode::Subtract => write!(f, "SUB"),
            OpCode::Multiply => write!(f, "MUL"),
            OpCode::Divide => write!(f, "DIV"),
            OpCode::GreaterThan => write!(f, "GT"),
            OpCode::LessThan => write!(f, "LT"),
            OpCode::Equal => write!(f, "EQ"),
            OpCode::NotEqual => write!(f, "NE"),
            OpCode::And => write!(f, "AND"),
            OpCode::Or => write!(f, "OR"),
            OpCode::Not => write!(f, "NOT"),
            OpCode::Jump(addr) => write!(f, "JUMP {}", addr),
            OpCode::JumpIfFalse(addr) => write!(f, "JUMP_IF_FALSE {}", addr),
            OpCode::JumpIfTrue(addr) => write!(f, "JUMP_IF_TRUE {}", addr),
            OpCode::JumpBackward(addr) => write!(f, "JUMP_BACKWARD {}", addr),
            OpCode::CallFunction(name, argc) => write!(f, "CALL_FUNC {} ({})", name, argc),
            OpCode::CallBuiltin(name, argc) => write!(f, "CALL_BUILTIN {} ({})", name, argc),
            OpCode::Return => write!(f, "RETURN"),
            OpCode::ReturnValue => write!(f, "RETURN_VALUE"),
            OpCode::BuildList(count) => write!(f, "BUILD_LIST {}", count),
            OpCode::BuildMap(count) => write!(f, "BUILD_MAP {}", count),
            OpCode::ListIndex => write!(f, "LIST_INDEX"),
            OpCode::MapIndex => write!(f, "MAP_INDEX"),
            OpCode::ListAppend(name) => write!(f, "LIST_APPEND {}", name),
            OpCode::MapStore(name) => write!(f, "MAP_STORE {}", name),
            OpCode::Print => write!(f, "PRINT"),
            OpCode::Pop => write!(f, "POP"),
            OpCode::Dup => write!(f, "DUP"),
            OpCode::Break => write!(f, "BREAK"),
            OpCode::Continue => write!(f, "CONTINUE"),
            OpCode::LoadNull => write!(f, "LOAD_NULL"),
        }
    }
}


