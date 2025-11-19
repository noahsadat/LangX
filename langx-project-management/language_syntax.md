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

#### For Loop
```
For each [variable] in [list]: [statement].
End for.
```
Example:
```
Set sum to 0.
Set numbers to [1, 2, 3, 4, 5].
For each num in numbers:
    Set sum to sum + num.
End for.
print sum.
```

#### Break Statement
Exit a loop early.
```
Break loop.
```
Example:
```
Set count to 0.
Repeat 10 times:
    Set count to count + 1.
    If count is greater than 5 then Break loop.
End repeat.
```

#### Continue Statement
Skip to the next iteration of a loop.
```
Continue to next iteration.
```
Example:
```
Set sum to 0.
Repeat 10 times:
    Set i to i + 1.
    If i is equal to 5 then Continue to next iteration.
    Set sum to sum + i.
End repeat.
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

Define [name] with parameters [param1], [param2], [param3]:
    [statements]
End definition.

Define [name] with parameters [param1], [param2], [param3], [param4]:
    [statements]
End definition.

Define [name] with parameters [param1], [param2], [param3], [param4], [param5]:
    [statements]
End definition.
```

**Note:** Parameters separated by commas. Functions support 0-5 parameters.

Examples:
```
Define greet with parameter name:
    print "Hello, ".
    print name.
End definition.

Define add with parameters a, b:
    Return a + b.
End definition.

Define sum_three with parameters a, b, c:
    Return a + b + c.
End definition.

Define multiply_four with parameters a, b, c, d:
    Return a * b * c * d.
End definition.
```

### Function Calls
```
Call [name].
Call [name] with [arg].
Call [name] with [arg1], [arg2].
Call [name] with [arg1], [arg2], [arg3].
Call [name] with [arg1], [arg2], [arg3], [arg4].
Call [name] with [arg1], [arg2], [arg3], [arg4], [arg5].
```

**Note:** Arguments separated by commas. Functions support 0-5 arguments.

Examples:
```
Call greet with "World".
Set sum to Call add with 5, 10.
Set total to Call sum_three with 1, 2, 3.
Set product to Call multiply_four with 2, 3, 4, 5.
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
[expr1] + [expr2]     # Addition (numbers) or concatenation (strings)
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

**String Concatenation:**
The `+` operator also works for string concatenation:
```
Set text to "Hello" + ", " + "World".     # = "Hello, World"
Set message to "The answer is " + 42.     # = "The answer is 42"
Set text2 to 100 + " percent".            # = "100 percent"
Set status to "Status: " + true.          # = "Status: true"
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
- **String concatenation**: `"Hello" + 42`, `100 + " percent"`
- **Logic**: `a and b`, `x or y`, `not flag`
- **Comparisons**: `x is greater than 5`
- **Function calls**: `Call add with 5, 10`, `Call string_length with "text"`
- **Parenthesized expressions**: `(2 + 3) * 4`
- **List literals**: `[1, 2, 3]`
- **List indexing**: `item 0 of list`

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

## Built-in Functions

### String Functions

#### `string_length`
Get the length of a string.
```
Call string_length with [string]
```
Example:
```
Set text to "Hello, World!".
Set len to Call string_length with text.
print len.  # Prints: 13
```

#### `substring`
Extract a substring from a string.
```
Call substring with [string], [start], [length]
```
Example:
```
Set text to "Hello, World!".
Set sub to Call substring with text, 0, 5.
print sub.  # Prints: "Hello"
```

#### `split`
Split a string into a list by a delimiter.
```
Call split with [string], [delimiter]
```
Example:
```
Set text to "a,b,c".
Set parts to Call split with text, ",".
# parts is now ["a", "b", "c"]
```

#### `join`
Join a list into a string with a delimiter.
```
Call join with [list], [delimiter]
```
Example:
```
Set list to ["a", "b", "c"].
Set result to Call join with list, ",".
# result is "a,b,c"

Set numbers to [1, 2, 3].
Set text to Call join with numbers, "-".
# text is "1-2-3"
```

#### `replace`
Replace all occurrences of a substring in a string.
```
Call replace with [string], [old], [new]
```
Example:
```
Set text to "Hello World".
Set result to Call replace with text, "World", "LangX".
# result is "Hello LangX"

Set text2 to "cat cat dog".
Set result2 to Call replace with text2, "cat", "dog".
# result2 is "dog dog dog"
```

### Math Functions

#### `abs`
Get the absolute value of a number.
```
Call abs with [number]
```
Example:
```
Set neg to 0 - 5.
Set abs_val to Call abs with neg.
print abs_val.  # Prints: 5
```

#### `min`
Get the minimum of two numbers.
```
Call min with [number1], [number2]
```
Example:
```
Set result to Call min with 10, 5.
print result.  # Prints: 5
```

#### `max`
Get the maximum of two numbers.
```
Call max with [number1], [number2]
```
Example:
```
Set result to Call max with 10, 5.
print result.  # Prints: 10
```

#### `pow`
Raise a number to a power (base^exponent). Exponent must be non-negative.
```
Call pow with [base], [exponent]
```
Example:
```
Set result to Call pow with 2, 8.
print result.  # Prints: 256
```

#### `sqrt`
Get the square root of a number (returns integer floor). Number must be non-negative.
```
Call sqrt with [number]
```
Example:
```
Set result to Call sqrt with 25.
print result.  # Prints: 5

Set result2 to Call sqrt with 20.
print result2.  # Prints: 4 (floor of 4.47...)
```

#### `round`
Round a number to the nearest integer. For integers, returns the number itself.
```
Call round with [number]
```
Example:
```
Set result to Call round with 42.
print result.  # Prints: 42
```

#### `floor`
Get the floor of a number. For integers, returns the number itself.
```
Call floor with [number]
```
Example:
```
Set result to Call floor with 42.
print result.  # Prints: 42
```

#### `ceil`
Get the ceiling of a number. For integers, returns the number itself.
```
Call ceil with [number]
```
Example:
```
Set result to Call ceil with 42.
print result.  # Prints: 42
```

## Future Syntax Extensions
- **String interpolation**: `"Hello, {name}!"`
- **Match expressions**: `Match [value] with [patterns]`
- **Lambda functions**: `Set func to function with x: Return x * 2.`
- **File I/O operations**: read_file, write_file
