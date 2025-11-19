# Known Issues

## Priority 1: Function Definition Issue with Regular + Variadic Parameters

**Status:** Open  
**Priority:** High  
**Date:** November 2025

### Description
Functions with both regular and variadic parameters (e.g., `Define func with parameters a, ...b:`) fail to be defined when the function body contains both:
1. A `Set` statement (variable assignment)
2. A `For` loop that iterates over the variadic parameter

### Symptoms
- Function definition appears to parse successfully
- Function is not stored/accessible when called
- Error: "Runtime error: Undefined function 'function_name'"

### Working Cases
- ✅ Variadic-only functions: `Define func with parameters ...args:`
- ✅ Regular + variadic with simple body (no Set + For combination)
- ✅ Functions with default parameters work correctly
- ✅ Functions with variadic parameters work when body doesn't have Set + For

### Failing Cases
```langx
Define join_strings with parameters separator, ...strings:
    Set result to "".
    For each item in strings:
        Set result to result + item.
    End for.
    Return result.
End definition.

Set x to Call join_strings with ", ", "a", "b", "c".
```

### Investigation Notes
- Parsing succeeds (no parse errors)
- Function definition statement executes (prints before function call work)
- Function is not found in environment when called
- Issue occurs specifically when function body has both `Set` statement and `For` loop using variadic parameter
- Order doesn't matter (Set before For or For before Set both fail)

### Next Steps
1. Investigate function definition execution path when body contains Set + For
2. Check if there's an error being silently caught during function storage
3. Verify AST cloning/storage for function bodies with this pattern
4. Add debug logging to function definition execution

### Test Case
See: `langx/src/tests/mod.rs::test_variadic_with_regular_parameters` (currently ignored)

