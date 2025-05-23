# Function Implementation Plan

## Overview
This document outlines the plan for implementing function definitions and calls in LangX.

## Syntax Design

### Function Definition
```
Define [name] with parameters [param1], [param2]:
    [statements]
End definition.
```

### Function Call
```
Call [name] with [arg1], [arg2].
```

## Implementation Steps

### 1. AST Updates [COMPLETED]
- [x] Add `FunctionDefinition` to the `Statement` enum:
  ```rust
  FunctionDefinition {
      name: String,
      parameters: Vec<String>,
      body: Vec<Statement>,
  }
  ```
- [x] Add `FunctionCall` to the `Expression` enum:
  ```rust
  FunctionCall {
      name: String,
      arguments: Vec<Expression>,
  }
  ```
- [x] Add `Return` to the `Statement` enum:
  ```rust
  Return(Option<Expression>)
  ```

### 2. Parser Updates [COMPLETED]
- [x] Update `grammar.lalrpop` to handle function definitions:
  ```
  "Define" <name:Identifier> "with" "parameters" <params:Comma<Identifier>> ":" <body:Statement*> "End" "definition" "." => {
      Statement::FunctionDefinition { name, parameters: params, body }
  }
  ```
- [x] Update `grammar.lalrpop` to handle function calls:
  ```
  "Call" <name:Identifier> "with" <args:Comma<Expression>> "." => {
      Expression::FunctionCall { name, arguments: args }
  }
  ```
- [x] Add support for return statements:
  ```
  "Return" <expr:Expression?> "." => {
      Statement::Return(expr)
  }
  ```

### 3. Interpreter Updates [COMPLETED]
- [x] Add function storage to the `Environment` struct:
  ```rust
  functions: HashMap<String, (Vec<String>, Vec<Statement>)>,
  ```
- [x] Implement function definition execution:
  ```rust
  match statement {
      Statement::FunctionDefinition { name, parameters, body } => {
          self.env.define_function(name, parameters, body);
      }
      // ...
  }
  ```
- [x] Implement function call evaluation:
  ```rust
  match expr {
      Expression::FunctionCall { name, arguments } => {
          let (params, body) = self.env.get_function(name)?;
          // Create new scope with arguments bound to parameters
          // Execute function body
          // Return result
      }
      // ...
  }
  ```
- [x] Implement return statement handling:
  ```rust
  match statement {
      Statement::Return(expr) => {
          // Set return value and signal to stop execution
      }
      // ...
  }
  ```

### 4. Environment Updates [COMPLETED]
- [x] Add scoping support to the `Environment` struct:
  ```rust
  parent: Option<Box<Environment>>,
  ```
- [x] Add methods for creating and managing nested scopes:
  ```rust
  fn new_scope(&self) -> Environment { ... }
  fn get(&self, name: &str) -> Option<Value> { ... } // Check local, then parent
  ```

### 5. Testing [COMPLETED]
- [x] Write unit tests for function definitions
- [x] Write unit tests for function calls
- [x] Write unit tests for return statements
- [x] Write integration tests with complete function examples

## Example Program
```
Define add with parameters a, b:
    Return a plus b.
End definition.

Define greet with parameters name:
    Set message to "Hello, ".
    Set full_greeting to message plus name.
    print full_greeting.
End definition.

Call greet with "World".
Set result to Call add with 5, 10.
print result.
```

## Status
✅ Implementation completed. Functions with parameters, return values, and proper scoping are now fully functional in LangX.

## Timeline
- AST and Parser updates: Completed
- Interpreter and Environment updates: Completed
- Testing and debugging: Completed
- Documentation: Completed

## Challenges and Considerations
1. Scoping rules for variables inside functions
2. Return value handling
3. Recursion support
4. Error handling for undefined functions
5. Type checking for function arguments 