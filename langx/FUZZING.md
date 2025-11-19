# Fuzzing Tests for LangX

LangX uses property-based testing with [proptest](https://docs.rs/proptest/) to find edge cases and bugs through randomized input generation.

## Overview

Fuzzing tests generate random inputs and verify that LangX components handle them correctly without panicking. This helps find:
- UTF-8 boundary issues
- Memory safety problems
- Edge cases in parsing and execution
- Unexpected input handling

## Running Fuzz Tests

```bash
# Run all fuzz tests
cargo test fuzz

# Run specific fuzz test
cargo test fuzz_lexer
cargo test fuzz_parser
cargo test fuzz_interpreter

# Run with more verbose output
cargo test fuzz -- --nocapture
```

## Fuzz Test Targets

### Lexer Fuzzing
- **`fuzz_lexer`**: Tests tokenization with random LangX-like inputs
- **`fuzz_random_strings`**: Tests tokenization with completely random strings
- **`prop_lexer_always_produces_tokens`**: Property test ensuring lexer never panics

### Parser Fuzzing
- **`fuzz_parser`**: Tests parsing with random LangX-like inputs
- **`fuzz_parser_recovery`**: Tests error recovery with multiple errors
- **`prop_parse_format_idempotent`**: Property test for parse consistency

### Interpreter Fuzzing
- **`fuzz_interpreter`**: Tests execution with random valid programs
- Verifies runtime errors are handled gracefully (no panics)

### Bytecode Compiler Fuzzing
- **`fuzz_bytecode_compiler`**: Tests bytecode compilation
- **`fuzz_bytecode_vm`**: Tests bytecode VM execution

### Utility Function Fuzzing
- **`prop_line_number_safe`**: Tests line number calculation with random positions
- **`prop_code_snippet_safe`**: Tests code snippet extraction with random inputs

## Bugs Found

Fuzzing has already discovered and fixed several bugs:

1. **UTF-8 Boundary Issue in String Literals** (Fixed)
   - Problem: Lexer panicked when slicing strings at non-char boundaries
   - Fix: Added `is_char_boundary` checks before string slicing

2. **UTF-8 Boundary Issue in Line Number Calculation** (Fixed)
   - Problem: `line_number_at_position` panicked on multi-byte UTF-8 characters
   - Fix: Added char boundary validation before slicing

## Configuration

Fuzz tests use proptest's default configuration with custom case counts:
- Most tests: 1000 cases
- Expensive tests (interpreter, VM): 300-500 cases
- Property tests: 200-1000 cases

To modify test counts, edit `src/tests/fuzz.rs` and adjust the `config.cases` values.

## Continuous Fuzzing

For continuous fuzzing, you can run:

```bash
# Run fuzz tests in a loop
while true; do cargo test fuzz --lib; done

# Or use a timeout to limit runtime
timeout 3600 cargo test fuzz --lib -- --test-threads=1
```

## Integration with CI

Fuzz tests are included in the standard test suite and run automatically with `cargo test`. They're designed to be fast enough for CI while still providing good coverage.

## Future Improvements

- Add more targeted fuzz tests for specific language features
- Increase test case counts for critical paths
- Add fuzzing for bytecode serialization/deserialization
- Consider using `cargo-fuzz` with libFuzzer for more intensive fuzzing

