# Arithmetic Operations Implementation Plan

## Overview
This document outlines the implementation of arithmetic operations in LangX using mathematical symbols.

## Current Status
- [x] **All arithmetic operations implemented with symbols (+ - * /)**
- [x] Addition operation
- [x] Subtraction operation
- [x] Multiplication operation
- [x] Division operation
- [x] Operator precedence (multiplication/division before addition/subtraction)
- [x] Parentheses for grouping expressions

## Major Design Decision (October 2025)
**Switched from English words to mathematical symbols for arithmetic operations.**

### Rationale:
1. **Eliminated parser ambiguity** - Using symbols for arithmetic and "and" exclusively for logic/function args
2. **Universal understanding** - Math symbols are recognized globally
3. **Cleaner syntax** - More concise and familiar to programmers
4. **Better precedence** - Standard mathematical precedence rules apply naturally

## Syntax Design

### Addition
```
[expr1] + [expr2]
```
Example: `Set x to 5 + 3.`

### Subtraction
```
[expr1] - [expr2]
```
Example: `Set y to 10 - 7.`

### Multiplication
```
[expr1] * [expr2]
```
Example: `Set z to 6 * 4.`

### Division
```
[expr1] / [expr2]
```
Example: `Set result to 20 / 5.`

### Operator Precedence
Standard mathematical precedence applies:
1. **Parentheses** - highest priority
2. **Multiplication and Division** - left to right
3. **Addition and Subtraction** - left to right

Examples:
```
Set a to 2 + 3 * 4.        # Result: 14 (not 20)
Set b to (2 + 3) * 4.      # Result: 20
Set c to 20 / 2 - 3.       # Result: 7
Set d to 20 / (2 - 3).     # Result: -20
```

## Implementation Steps

### 1. Lexer Updates [COMPLETED]
- [x] Replaced word tokens with symbol tokens
- [x] Added `+` token (Plus)
- [x] Added `-` token (Minus)
- [x] Added `*` token (Times)
- [x] Added `/` token (Divide)
- [x] Kept "times" keyword for `Repeat N times:`

### 2. Parser Updates [COMPLETED]
- [x] Updated grammar to use symbol tokens
- [x] Implemented proper operator precedence
- [x] Used fold-based approach for left-associativity
- [x] Eliminated all LALRPOP ambiguity warnings
- [x] Added parentheses support

### 3. Interpreter Updates [COMPLETED]
- [x] Updated evaluation logic for new AST structure
- [x] Added zero-division error handling
- [x] Ensured correct precedence evaluation

### 4. Testing [COMPLETED]
- [x] Unit tests for all operators
- [x] Tests for operator precedence
- [x] Tests for parenthesized expressions
- [x] Integration tests with example programs

### 5. Documentation [COMPLETED]
- [x] Updated all example files to use new syntax
- [x] Updated language reference
- [x] Updated project descriptions

## Benefits Achieved
1. ✅ **Zero parser ambiguities** - Clean LR(1) grammar
2. ✅ **Familiar syntax** - Standard math notation
3. ✅ **Clear separation** - Symbols for math, words for logic
4. ✅ **Proper precedence** - Mathematical rules apply
5. ✅ **Maintainable** - Easy to extend and debug

## Future Enhancements
1. Modulo operator (%)
2. Exponentiation operator (**)
3. Unary operations (negation: -x)
4. Bitwise operators (if needed)
5. Type conversion operations
6. String concatenation with +

## Lessons Learned
- Hybrid syntax (symbols + words) can be superior to pure English
- Parser ambiguity should drive design decisions
- Sometimes the "programming way" is the right way
- Mathematical symbols are a universal language
