# LangX Language Syntax Reference

## Current Language Features

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

### Print Statements
```
print [expression].
```
Example:
```
print "Hello, world!".
print x.
```

### Function Definitions
```
Define [name]:
    [statements]
End definition.

Define [name] with parameter [param]:
    [statements]
End definition.

Define [name] with parameters [param1] and [param2]:
    [statements]
End definition.
```
Example:
```
Define greet with parameter name:
    print "Hello, ".
    print name.
End definition.
```

### Function Calls
```
Call [name].
Call [name] with [arg].
Call [name] with [arg1] and [arg2].
```
Example:
```
Call greet with "World".
```

### Return Statements
```
Return [expression].
```
Example:
```
Return x plus 5.
```

### Arithmetic Operations
```
[expr1] plus [expr2]
[expr1] minus [expr2]
[expr1] times [expr2]
[expr1] divided by [expr2]
```

Example:
```
Set result to 10 plus 5.
Set difference to 20 minus 7.
Set product to 6 times 7.
Set quotient to 100 divided by 4.
```

### Expressions
Currently supported expression types:
- Numbers (integers): `42`
- Strings: `"Hello, world!"`
- Variables: `x`
- Comparisons: `x is greater than 5`
- Function calls: `Call add with 5 and 10`
- Arithmetic operations: `x plus 5`, `a minus b`, `2 times 3`, `10 divided by 2`

### Operators
Currently supported operators:
- `is greater than` - Comparison operator
- `plus` - Addition operator
- `minus` - Subtraction operator
- `times` - Multiplication operator
- `divided by` - Division operator

## Planned Language Features

### More Comparison Operators
```
[expr1] is equal to [expr2]
[expr1] is not equal to [expr2]
[expr1] is less than [expr2]
```

### Logical Operators
```
[expr1] and [expr2]
[expr1] or [expr2]
not [expr]
```

### Block Statements
```
Begin block:
    [statement1]
    [statement2]
End block.
```

### Lists
```
Set [variable] to list of [item1], [item2], [item3].
Add [item] to [list].
Get item at position [index] from [list].
```

### Input/Output
```
Set [variable] to input from user.
Write [expression] to file [filename].
Set [variable] to contents of file [filename].
```

## Syntax Conventions
- Statements end with a period (`.`)
- Blocks are indented for readability
- Lists of items are separated by commas
- Variable names are case-sensitive
- Keywords (Set, If, Repeat, etc.) are capitalized at the beginning of statements

## Grammar Summary
LangX uses a formal grammar defined with LALRPOP. The language is designed to be:
1. Readable as English
2. Unambiguous for parsing
3. Deterministic in execution
4. Extensible for future features 

## Operator Precedence (Coming Soon)
In the current implementation, operators are evaluated from left to right without precedence. Future updates will implement proper operator precedence, where:
1. Multiplication and division have higher precedence than addition and subtraction
2. Operators at the same precedence level are evaluated from left to right
3. Parentheses can be used to override default precedence 