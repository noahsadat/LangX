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
If [condition] then [statement].
```

Example:
```
If x is greater than 5 then print "x is large".
```

### Loops

#### Repeat Loop
```
Repeat [count] times: [statement].
End repeat.
```

Example:
```
Repeat 3 times: print "Hello".
End repeat.
```

#### While Loop
```
While [condition]: [statement].
End while.
```

Example:
```
Set x to 0.
While x is less than 5:
    print x.
    Set x to x + 1.
End while.
```

### Functions

Functions support 0-5 parameters:
```
Define [name]:
    [statements]
End definition.

Define [name] with parameter [param]:
    [statements]
End definition.

Define [name] with parameters [param1], [param2]:
    [statements]
End definition.

Define [name] with parameters [param1], [param2], [param3]:
    [statements]
End definition.

Call [name] with [arg1], [arg2], [arg3].
```

Example:
```
Define add with parameters a, b:
    Return a + b.
End definition.

Set result to Call add with 5, 10.
print result.
```

### Lists/Arrays

```
Set list to [1, 2, 3, 4, 5].
Set first to item 0 of list.
Add 6 to list.
```

Example:
```
Set numbers to [10, 20, 30].
Set first to item 0 of numbers.
Add 40 to numbers.
print numbers.  # Prints: [10, 20, 30, 40]
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

## Current Capabilities (December 2025)

LangX supports:
- Variables and assignment
- Arithmetic with symbols: `+`, `-`, `*`, `/`
- Logical expressions with words: `and`, `or`, `not`
- Comparisons: `is greater than`, `is less than`, `is equal to`, `is not equal to`
- Parentheses and operator precedence
- Conditional statements: `If condition then statement`
- Loops: `Repeat N times: ... End repeat.`, `While condition: ... End while.`
- Functions: 0-5 parameters with return values
- Function calls: `Call func with arg1, arg2, arg3`
- String concatenation: `"Hello" + 42`, `100 + " percent"`
- Lists/Arrays: Creation, indexing, appending
- Built-in functions: `string_length`, `substring`
- Print statements
- Comments: `# comment`

### String Concatenation
The `+` operator works for both arithmetic and string concatenation:
```
Set text to "Hello" + ", " + "World".     # = "Hello, World"
Set message to "The answer is " + 42.     # = "The answer is 42"
Set text2 to 100 + " percent".            # = "100 percent"
Set status to "Status: " + true.         # = "Status: true"
```

### Extended Function Support
Functions now support up to 5 parameters:
```
Define sum_three with parameters a, b, c:
    Return a + b + c.
End definition.

Set total to Call sum_three with 1, 2, 3.
```

### Built-in String Functions
```
# Get string length
Set len to Call string_length with "Hello, World!".

# Extract substring
Set sub to Call substring with "Hello, World!", 0, 5.
```

### Example
```
Define add with parameters a, b:
    Return a + b.
End definition.

Set sum to Call add with 5, 10.
print sum.

# String concatenation
Set greeting to "Hello, " + "World!".
print greeting.

# Built-in functions
Set text to "LangX".
Set len to Call string_length with text.
Set sub to Call substring with text, 0, 4.
print len.
print sub.
```

## Recent Improvements (December 2025)
- ✅ Enhanced error messages with line numbers and code snippets
- ✅ Extended function support to 5 parameters (was 0-2)
- ✅ String concatenation with multiple types (string + number, number + string, string + boolean)
- ✅ Built-in string functions (`string_length`, `substring`)
- ✅ Fixed parentheses precedence issue
- ✅ Comprehensive test suite (20+ new tests)
- ✅ Fixed all example file syntax issues
- ✅ Improved test coverage to ~70% 