# Arithmetic Operations Implementation Plan

## Overview
This document outlines the plan for implementing arithmetic operations in LangX.

## Current Status
- [x] Addition operation is implemented and working
- [x] Subtraction operation
- [x] Multiplication operation
- [x] Division operation

## Implementation Challenges
We encountered issues with the LALRPOP grammar when trying to implement all operations at once with proper operator precedence. The current solution is to implement operations one by one, starting with addition.

## Syntax Design

### Addition (Implemented)
```
[expr1] plus [expr2]
```

### Subtraction (Implemented)
```
[expr1] minus [expr2]
```

### Multiplication (Implemented)
```
[expr1] times [expr2]
```

### Division (Implemented)
```
[expr1] divided by [expr2]
```

## Implementation Steps

### 1. Addition (Completed)
- [x] Added Plus token to lexer
- [x] Added Plus binary operator to AST
- [x] Updated grammar to parse addition expressions
- [x] Updated interpreter to evaluate addition expressions
- [x] Added tests for addition
- [x] Created example program (addition.lx)

### 2. Subtraction (Completed)
- [x] Update the grammar to handle subtraction
- [x] Update the interpreter to evaluate subtraction expressions
- [x] Add tests for subtraction
- [x] Update example programs

### 3. Multiplication (Completed)
- [x] Update the grammar to handle multiplication
- [x] Update the interpreter to evaluate multiplication expressions
- [x] Add tests for multiplication
- [x] Update example programs

### 4. Division (Completed)
- [x] Update the grammar to handle division
- [x] Update the interpreter to evaluate division expressions with zero-division checks
- [x] Add tests for division
- [x] Update example programs

### 5. Operator Precedence (Next)
- [ ] Reorganize grammar to implement correct operator precedence
  - Multiplication and division have higher precedence than addition and subtraction
  - Left-to-right associativity within the same precedence level
- [ ] Add tests for complex expressions with mixed operators

### 6. Documentation
- [ ] Update language syntax documentation
- [ ] Add examples showing all arithmetic operations
- [ ] Document operator precedence rules

## Future Enhancements
1. Parentheses for grouping expressions
2. Unary operations (negation)
3. Support for more complex expressions (variables, function calls in all positions)
4. String concatenation with the plus operator
5. Type conversion operations

## Timeline
- Addition: Completed
- Subtraction: Completed
- Multiplication: Completed
- Division: Completed
- Operator precedence: 2 days
- Documentation: 1 day

## Lessons Learned
- Implementing a grammar with operator precedence in LALRPOP is challenging
- A simplified approach (implementing one operation at a time) was more successful
- Once all operations are working independently, we can focus on improving the grammar structure for operator precedence 