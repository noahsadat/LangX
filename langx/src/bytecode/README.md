# LangX Bytecode Compiler

This module implements a bytecode compiler and virtual machine for LangX programs.

## Overview

The bytecode compiler converts LangX AST (Abstract Syntax Tree) into a stack-based bytecode format, which is then executed by a virtual machine (VM). This provides a foundation for performance optimizations and future features like JIT compilation.

## Architecture

### Components

1. **Chunk** (`chunk.rs`): Represents compiled bytecode with:
   - Instruction sequence (`code: Vec<OpCode>`)
   - Constant pool (`constants: Vec<Value>`)
   - Line number information for debugging

2. **Compiler** (`compiler.rs`): Converts AST to bytecode:
   - Compiles expressions to bytecode instructions
   - Compiles statements to bytecode sequences
   - Handles function definitions and compiles function bodies
   - Manages control flow (loops, conditionals, jumps)

3. **VM** (`vm.rs`): Stack-based virtual machine:
   - Executes bytecode instructions
   - Manages call stack for function calls
   - Handles variable scoping and environments
   - Implements built-in functions

## Instruction Set

The bytecode uses a stack-based instruction set with the following categories:

### Constants
- `LoadConstant(index)` - Load constant from constant pool

### Variables
- `LoadVariable(name)` - Load variable onto stack
- `StoreVariable(name)` - Store top of stack to variable

### Arithmetic Operations
- `Add`, `Subtract`, `Multiply`, `Divide`

### Comparisons
- `GreaterThan`, `LessThan`, `Equal`, `NotEqual`

### Logical Operations
- `And`, `Or`, `Not`

### Control Flow
- `Jump(address)` - Unconditional jump
- `JumpIfFalse(address)` - Jump if top of stack is false
- `JumpIfTrue(address)` - Jump if top of stack is true
- `JumpBackward(address)` - Jump backward (for loops)

### Functions
- `CallFunction(name, argc)` - Call user-defined function
- `CallBuiltin(name, argc)` - Call built-in function
- `Return` - Return from function (no value)
- `ReturnValue` - Return with value from stack

### Data Structures
- `BuildList(count)` - Build list from N items on stack
- `BuildMap(count)` - Build map from N key-value pairs
- `ListIndex` - Pop index and list, push list[index]
- `MapIndex` - Pop key and map, push map[key]
- `ListAppend(name)` - Append value to list variable
- `MapStore(name)` - Store value in map at key

### Other
- `Print` - Pop value and print it
- `Pop` - Pop and discard top of stack
- `Dup` - Duplicate top of stack
- `Break` - Break out of loop
- `Continue` - Continue to next iteration
- `LoadNull` - Push null value

## Usage

### From Command Line

```bash
# Compile and run with bytecode
./target/release/langx --bytecode examples/hello.lx

# Or use the short flag
./target/release/langx -b examples/hello.lx
```

### Programmatically

```rust
use langx::{parser, bytecode};

// Parse source code
let program = parser::parse(source)?;

// Compile to bytecode
let mut compiler = bytecode::Compiler::new();
let chunk = compiler.compile(&program)?;
let functions = compiler.get_functions().clone();

// Execute bytecode
let mut vm = bytecode::VM::new();
vm.execute(chunk, functions)?;
```

## Current Status

### ✅ Implemented Features
- All expression types (numbers, strings, booleans, variables, operations)
- All statement types (assignments, conditionals, loops, prints)
- Function definitions and calls (non-recursive)
- Built-in functions (string, math, I/O, time/date)
- Data structures (lists, maps)
- Control flow (if/then, repeat, while, for)
- Break and continue statements
- Variable scoping and environments

### 🚧 Known Limitations
- **Recursive function calls**: Currently fail with "Undefined function" error. This is a known issue being debugged. For recursive functions, use the standard interpreter mode.

### 🔮 Future Improvements
- Recursive function call support
- Bytecode optimization passes
- JIT compilation
- WebAssembly target
- Performance profiling and optimization

## Performance

The bytecode compiler provides a foundation for performance improvements:
- Eliminates repeated AST traversal
- Enables instruction-level optimizations
- Prepares for JIT compilation
- Allows for better caching and precompilation

Benchmarks comparing interpreter vs bytecode modes are planned for future releases.

## Implementation Details

### Function Compilation

Functions are compiled in two passes:
1. **Collection**: All function definitions are collected into a functions map
2. **Compilation**: Each function body is compiled to bytecode, with access to all function definitions for nested calls

### Call Stack Management

The VM maintains a call stack (`CallFrame`) that stores:
- Return address
- Local environment (for variable scoping)
- Local chunk (for returning to caller's bytecode)

### Environment Scoping

Each function call creates a new environment with the parent environment, allowing proper variable scoping and closure-like behavior.

## Testing

Test the bytecode compiler with:

```bash
# Run all tests
cargo test

# Test specific bytecode features
./target/release/langx --bytecode examples/test_bytecode.lx
./target/release/langx --bytecode examples/test_functions_bytecode.lx
./target/release/langx --bytecode examples/test_nested_calls.lx
```

## Contributing

When adding new language features:
1. Add corresponding opcodes in `chunk.rs`
2. Update compiler to emit new opcodes in `compiler.rs`
3. Implement opcode execution in `vm.rs`
4. Add tests for the new feature

