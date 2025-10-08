# LangX

LangX is a programming language that interprets structured English as executable code. Built entirely in Rust, it parses and executes English-like statements deterministically without relying on AI or machine learning at runtime.

## Features

- English-like syntax for better readability
- Variables and assignments
- Conditional statements
- Loops
- Functions with parameters and return values
- Print statements

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version)

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/langx.git
cd langx
cargo build --release
```

## Usage

### Running the REPL

To start an interactive REPL session:

```bash
cargo run
```

### Running a LangX File

To execute a LangX program from a file:

```bash
cargo run -- examples/hello.lx
```

## Language Syntax

### Variable Assignment

```
Set [variable] to [expression].
```

Example:
```
Set x to 10.
Set greeting to "Hello, world!".
```

### Conditional Statements

```
If [condition], [statement].
```

Example:
```
If x is greater than 5, print "x is large".
```

### Loops

```
Repeat [count] times: [statement].
```

Example:
```
Repeat 3 times: print "Hello".
```

### Functions

```
Define [name] with parameters [param1], [param2]:
    [statements]
End definition.

Call [name] with [arg1], [arg2].
```

Example:
```
Define add with parameters a, b:
    Return a.
End definition.

Set result to Call add with 5, 10.
print result.
```

### Print Statements

```
print [expression].
```

Example:
```
print "Hello, world!".
print x.
```

## Testing

### Running Tests

To run all tests:

```bash
cargo test
```

### Writing Tests

You can write tests for your LangX programs in Rust. Here's an example:

```rust
#[test]
fn test_variable_assignment() {
    let source = "Set x to 42. print x.";
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    // Capture stdout and run the program
    let output = with_captured_stdout(|| {
        interpreter.interpret(&program).unwrap();
    });
    
    assert_eq!(output.trim(), "42");
}
```

## Examples

Check out the `examples/` directory for sample LangX programs:

- `hello.lx` - Basic variable assignment and printing
- `functions.lx` - Function definitions and calls

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with Rust
- Uses Logos for lexical analysis
- Uses LALRPOP for parsing 

## Current Capabilities (May 2025)

LangX supports:
- Variables and assignment
- Arithmetic and logical expressions (plus, minus, times, divided by, and, or, not, comparisons)
- Parentheses and operator precedence
- Conditional statements (if)
- Loops (repeat)
- Functions (0, 1, or 2 parameters)
- Function calls with natural 'and' separator (e.g., Call add with 5 and 10)
- Print statements

### Function Calls and Logical 'and'
- To call a function with two arguments: `Call add with 5 and 10` (arguments must be simple values, variables, or parenthesized expressions)
- To use logical 'and': `Set result to a and b`
- To pass a logical expression as a function argument, use parentheses: `Call print_results with (a and b) and (a or b)`

### Example
```
Define add with parameters a and b:
    Return a plus b.
End definition.

Set a to true.
Set b to false.

Set sum to Call add with 5 and 10.
print sum.

Set logic_result to a and b.
print logic_result.

Define print_results with parameters first and second:
    print first.
    print second.
End definition.

Call print_results with (a and b) and (a or b).
```

## Recent Improvements
- Operator precedence and parentheses
- Logical and comparison operators
- Natural function call syntax with 'and' (unambiguous for user code)
- Documentation and examples updated 