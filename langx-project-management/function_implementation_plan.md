# Function Implementation Plan

## Overview
This document outlines the implementation of function definitions and calls in LangX with the updated syntax.

## Current Status
✅ **Fully implemented and operational**

## Syntax Design

### Function Definition
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
```

**Note:** Parameters are separated by **commas**, not "and".

### Function Call
```
Call [name].
Call [name] with [arg].
Call [name] with [arg1], [arg2].
```

**Note:** Arguments are separated by **commas**, not "and".

## Key Syntax Change (October 2025)

### Before:
```
Define add with parameters a and b:
    Return a plus b.
End definition.

Call add with 5 and 10.
```

### After:
```
Define add with parameters a, b:
    Return a + b.
End definition.

Call add with 5, 10.
```

### Why Commas?
1. **Eliminates ambiguity** - "and" is now exclusively for logic
2. **Standard convention** - Matches most programming languages
3. **Clear separation** - Comma clearly separates arguments
4. **Parser friendly** - No conflicts with logical operators

## Implementation Details

### 1. AST Structure [COMPLETED]
```rust
Statement::FunctionDefinition {
    name: String,
    parameters: Vec<String>,
    body: Vec<Statement>,
}

Expression::FunctionCall {
    name: String,
    arguments: Vec<Expression>,
}

Statement::Return(Option<Expression>)
```

### 2. Parser Grammar [COMPLETED]
```lalrpop
// No parameters
"Define" <name:Identifier> ":" <body:Statement*> "End" "definition" "."

// One parameter
"Define" <name:Identifier> "with" "parameter" <param:Identifier> ":" <body:Statement*> "End" "definition" "."

// Two parameters (comma separated)
"Define" <name:Identifier> "with" "parameters" <param1:Identifier> "," <param2:Identifier> ":" <body:Statement*> "End" "definition" "."

// Function calls (comma separated arguments)
"Call" <name:Identifier> => ...
"Call" <name:Identifier> "with" <arg:PrimaryExpression> => ...
"Call" <name:Identifier> "with" <arg1:PrimaryExpression> "," <arg2:PrimaryExpression> => ...
```

### 3. Interpreter Implementation [COMPLETED]
- [x] Function storage in environment
- [x] Scoping with nested environments
- [x] Parameter binding to arguments
- [x] Return value handling
- [x] Proper scope cleanup

### 4. Features Implemented [COMPLETED]
- [x] 0, 1, and 2 parameter functions
- [x] Return statements with expressions
- [x] Function calls as expressions
- [x] Nested function calls
- [x] Proper variable scoping
- [x] Error handling for undefined functions
- [x] Argument count validation

## Example Programs

### Basic Function
```
Define greet with parameter name:
    print "Hello, ".
    print name.
End definition.

Call greet with "World".
```

### Function with Return Value
```
Define add with parameters a, b:
    Return a + b.
End definition.

Set result to Call add with 5, 10.
print result.  # Prints: 15
```

### Nested Function Calls
```
Define square with parameter x:
    Return x * x.
End definition.

Define add with parameters a, b:
    Return a + b.
End definition.

# Complex expression with function args in parentheses
Set result to Call add with 5, (Call square with 3).
print result.  # Prints: 14
```

### Function with Logic
```
Define max with parameters a, b:
    If a is greater than b then Return a.
    Return b.
End definition.

Set larger to Call max with 10, 20.
print larger.  # Prints: 20
```

## Argument Restrictions

For clarity and to avoid ambiguity, function arguments must be:
1. **Simple values**: Numbers, booleans, strings
2. **Variables**: Identifiers
3. **Parenthesized expressions**: `(a and b)`, `(x + y * z)`

**Examples:**
```
# ✅ Simple arguments
Call func with 5, 10.
Call func with x, y.

# ✅ Parenthesized complex expressions
Call func with (a and b), (x or y).
Call func with (2 + 3), (x * y).

# ✅ Nested function calls (parenthesized)
Call func with (Call other with 5), 10.
```

## Testing [COMPLETED]
- [x] Unit tests for function definitions
- [x] Unit tests for function calls
- [x] Unit tests for return statements
- [x] Integration tests with complete examples
- [x] Edge case testing (undefined functions, wrong arg count)
- [x] Scoping tests (variable shadowing, nested scopes)

## Status
✅ **Fully implemented and operational**

All function features are working correctly with the new comma-separated syntax.

## Future Enhancements
1. **Variadic functions** - Functions with any number of arguments
2. **Default parameters** - Optional parameters with default values
3. **Named arguments** - Call functions with parameter names
4. **Higher-order functions** - Functions that take/return functions
5. **Closures** - Functions that capture their environment
6. **Recursion optimization** - Tail call optimization

## Timeline
- AST and Parser updates: ✅ Completed
- Interpreter and Environment updates: ✅ Completed  
- Testing and debugging: ✅ Completed
- Syntax migration (and→comma): ✅ Completed
- Documentation: ✅ Completed

## Lessons Learned
1. **Syntax clarity matters** - Commas are clearer than "and" for lists
2. **Parser constraints drive design** - Ambiguity must be eliminated
3. **Scoping is crucial** - Proper variable scoping prevents bugs
4. **Return handling** - Need special control flow for returns
5. **Hybrid syntax works** - Mix symbols and words strategically
