# LangX Language Syntax Reference

## Language Philosophy
LangX combines the clarity of English with the precision of mathematical symbols, creating a hybrid syntax that is both readable and unambiguous.

## Current Language Features

### Variable Assignment
```
Set [variable] to [expression].
```
Examples:
```
Set x to 10.
Set greeting to "Hello, world!".
Set result to 5 + 3 * 2.
```

### Conditional Statements
```
If [condition] then [statement].
```
Examples:
```
If x is greater than 5 then print "x is large".
If flag then print "Flag is true".
```

**Note:** Uses `then`, not comma.

### Loops

#### Repeat Loop
```
Repeat [count] times: [statement].
```
Example:
```
Repeat 3 times: print "Hello".
```

#### While Loop
```
While [condition]: [statement].
```
Example:
```
Set x to 0.
While x is less than 5:
    print x.
    Set x to x + 1.
```

### Print Statements
```
print [expression].
```
Examples:
```
print "Hello, world!".
print x.
print 5 + 3.
```

### Comments
```
# This is a comment
```
Lines starting with `#` are ignored by the parser.

### Function Definitions
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

**Note:** Parameters separated by commas.

Examples:
```
Define greet with parameter name:
    print "Hello, ".
    print name.
End definition.

Define add with parameters a, b:
    Return a + b.
End definition.
```

### Function Calls
```
Call [name].
Call [name] with [arg].
Call [name] with [arg1], [arg2].
```

**Note:** Arguments separated by commas.

Examples:
```
Call greet with "World".
Set sum to Call add with 5, 10.
```

### Return Statements
```
Return [expression].
Return.
```
Examples:
```
Return x + 5.
Return true.
Return.
```

### Lists/Arrays

#### List Literals
```
Set [variable] to [[item1], [item2], ...].
```
Examples:
```
Set numbers to [1, 2, 3, 4, 5].
Set mixed to [1, "hello", true].
Set empty to [].
```

#### List Indexing
```
Set [variable] to item [index] of [list].
```
Examples:
```
Set first to item 0 of numbers.
Set second to item 1 of numbers.
```

**Note:** List indices start at 0. The index must be a number or a parenthesized expression.

#### List Append
```
Add [value] to [list].
```
Example:
```
Set list to [1, 2].
Add 3 to list.
# list is now [1, 2, 3]
```

## Expressions and Operators

### Arithmetic Operations (Symbols)
```
[expr1] + [expr2]     # Addition
[expr1] - [expr2]     # Subtraction
[expr1] * [expr2]     # Multiplication
[expr1] / [expr2]     # Division
```

Examples:
```
Set result to 10 + 5.
Set difference to 20 - 7.
Set product to 6 * 7.
Set quotient to 100 / 4.
Set complex to 2 + 3 * 4.      # = 14 (precedence)
Set grouped to (2 + 3) * 4.    # = 20 (parentheses)
```

### Logical Operations (Words)
```
[expr1] and [expr2]   # Logical AND
[expr1] or [expr2]    # Logical OR
not [expr]            # Logical NOT
```

Examples:
```
Set flag to true and false.
Set result to a or b.
Set inverted to not x.
Set complex to (a and b) or (not c).
```

### Comparison Operators
```
[expr1] is greater than [expr2]
[expr1] is less than [expr2]
[expr1] is equal to [expr2]
[expr1] is not equal to [expr2]
```

Examples:
```
If x is greater than 5 then print "Large".
If a is equal to b then print "Equal".
Set flag to x is less than 10.
```

### Boolean Literals
```
true
false
```

Examples:
```
Set flag to true.
Set active to false.
```

### Expression Types
Supported expression types:
- **Numbers** (integers): `42`, `0`, `-5`
- **Strings**: `"Hello, world!"`
- **Booleans**: `true`, `false`
- **Variables**: `x`, `myVar`
- **Arithmetic**: `5 + 3`, `x * y`, `(a + b) / c`
- **Logic**: `a and b`, `x or y`, `not flag`
- **Comparisons**: `x is greater than 5`
- **Function calls**: `Call add with 5, 10`
- **Parenthesized expressions**: `(2 + 3) * 4`

### Operator Precedence
**From highest to lowest:**
1. **Parentheses**: `( )`
2. **Unary operators**: `not`
3. **Multiplication & Division**: `*`, `/` (left to right)
4. **Addition & Subtraction**: `+`, `-` (left to right)
5. **Comparisons**: `is greater than`, `is less than`, `is equal to`, `is not equal to`
6. **Logical AND**: `and`
7. **Logical OR**: `or`

Examples:
```
2 + 3 * 4           # = 14 (mult first)
(2 + 3) * 4         # = 20 (parens first)
5 is greater than 3 and 2 is less than 4   # = true
not false or true   # = true (not first, then or)
```

## Syntax Design Principles

### Hybrid Approach
LangX uses **symbols for arithmetic** and **words for logic**:

**Why symbols for math?**
- Universal understanding
- Concise and familiar
- Eliminates parser ambiguity
- Standard precedence rules apply

**Why words for logic?**
- Clearer intent in conditions
- More readable than && and ||
- Natural for English-like syntax
- "and", "or", "not" are unambiguous

**Why commas for arguments?**
- Standard programming convention
- Clear separation of items
- No conflict with "and" operator
- Familiar to all programmers

### Ambiguity Resolution
The comma separator for function arguments was chosen to eliminate ambiguity:
- ✅ `Call add with 5, 10` - Clear
- ❌ `Call add with 5 and 10` - Ambiguous (and = separator or operator?)

Parentheses required for complex expressions as function arguments:
- ✅ `Call func with (a and b), (x or y)` - Clear
- ❌ `Call func with a and b, x or y` - Ambiguous

## Syntax Conventions
- **Statements end with period** (`.`)
- **Blocks use indentation** for readability
- **Keywords capitalized** at start: `Set`, `If`, `Define`, etc.
- **Variable names** are case-sensitive
- **Comments** start with `#`
- **Strings** use double quotes: `"text"`

## Grammar Summary
LangX uses a formal grammar defined with LALRPOP. The language is:
1. **Readable** - English-like structure
2. **Unambiguous** - Clean LR(1) grammar, zero conflicts
3. **Deterministic** - Predictable execution
4. **Extensible** - Easy to add features
5. **Hybrid** - Best of symbols and words

## Example Program
```
# Define a function
Define max with parameters a, b:
    If a is greater than b then Return a.
    Return b.
End definition.

# Use arithmetic and logic
Set x to 10 + 5.
Set y to 20.
Set flag to x is less than y.

# Call function
Set larger to Call max with x, y.

# Print results
If flag then print "x is smaller".
print larger.

# Loop with arithmetic
Repeat 3 times: print x * 2.
```

## Future Syntax Extensions
- **While loops**: `While [condition]: [statement]`
- **For loops**: `For each [item] in [list]: [statement]`
- **Lists**: `Set list to [1, 2, 3, 4, 5].`
- **String interpolation**: `"Hello, {name}!"`
- **Match expressions**: `Match [value] with [patterns]`
- **Lambda functions**: `Set func to function with x: Return x * 2.`
