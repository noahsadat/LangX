//! # Bytecode Compiler and Virtual Machine
//!
//! This module provides a bytecode compiler and virtual machine for LangX programs.
//! It converts AST into a stack-based bytecode format and executes it efficiently.
//!
//! ## Overview
//!
//! The bytecode compiler provides:
//! - **Compiler**: Converts AST to bytecode instructions
//! - **VM**: Stack-based virtual machine for executing bytecode
//! - **Chunk**: Bytecode container with instructions and constants
//!
//! ## Usage
//!
//! ```rust
//! use langx::{parser, bytecode};
//!
//! // Parse source code
//! let source = "Set x to 10. print x.";
//! let program = parser::parse(source)?;
//!
//! // Compile to bytecode
//! let mut compiler = bytecode::Compiler::new();
//! let chunk = compiler.compile(&program)?;
//! let functions = compiler.get_functions().clone();
//!
//! // Execute bytecode
//! let mut vm = bytecode::VM::new();
//! vm.execute(chunk, functions)?;
//! # Ok::<(), String>(())
//! ```
//!
//! See the [bytecode README](bytecode/README.md) for detailed documentation.

pub mod compiler;
pub mod vm;
pub mod chunk;

pub use chunk::{Chunk, OpCode};
pub use compiler::Compiler;
pub use vm::VM;

