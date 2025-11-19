//! # LangX Programming Language
//!
//! LangX is a programming language that interprets structured English as executable code.
//! Built entirely in Rust, it parses and executes English-like statements deterministically
//! without relying on AI or machine learning at runtime.
//!
//! ## Overview
//!
//! LangX combines the clarity of English with the precision of mathematical symbols,
//! creating a hybrid syntax that is both readable and unambiguous.
//!
//! ## Core Components
//!
//! - **[`lexer`](lexer/index.html)**: Lexical analysis - converts source code into tokens
//! - **[`parser`](parser/index.html)**: Parsing - converts tokens into an Abstract Syntax Tree (AST)
//! - **[`ast`](ast/index.html)**: Abstract Syntax Tree - represents the structure of LangX programs
//! - **[`interpreter`](interpreter/index.html)**: Execution engine - evaluates AST nodes
//! - **[`bytecode`](bytecode/index.html)**: Bytecode compiler and VM - compiles AST to bytecode and executes it
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use langx::{parser, interpreter};
//!
//! // Parse source code
//! let source = "Set x to 10. print x.";
//! let program = parser::parse(source)?;
//!
//! // Execute the program
//! let mut interpreter = interpreter::Interpreter::new();
//! interpreter.interpret(&program)?;
//! # Ok::<(), String>(())
//! ```
//!
//! ## Example: Using the Bytecode Compiler
//!
//! ```rust,no_run
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

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod interpreter;
pub mod bytecode;

