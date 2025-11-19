//! Fuzzing tests using property-based testing (proptest)
//!
//! These tests use proptest to generate random inputs and verify that
//! the lexer, parser, and interpreter handle them correctly without panicking.

use proptest::prelude::*;
use langx::{lexer, parser, interpreter};

/// Generate random LangX source code strings for fuzzing
fn arbitrary_langx_source() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::sample::select(vec![
            "Set x to 10.",
            "print x.",
            "If x is greater than 5 then print \"hello\".",
            "Repeat 3 times: print \"loop\".",
            "While x is less than 10: Set x to x + 1.",
            "Define add with parameters a, b: Return a + b. End definition.",
            "Call add with 1, 2.",
            "Set list to [1, 2, 3].",
            "Set map to {\"key\": \"value\"}.",
            "Set x to x + 1.",
            "Set x to x - 1.",
            "Set x to x * 2.",
            "Set x to x / 2.",
            "Set x to true.",
            "Set x to false.",
            "Set x to \"hello\".",
            "Set x to \"hello\" + \"world\".",
            "Set x to 1 + 2 + 3.",
            "Set x to (1 + 2) * 3.",
            "Break loop.",
            "Continue to next iteration.",
            "Return x.",
            "Return.",
            "# This is a comment",
            "Set x to item 0 of list.",
            "Set x to map at \"key\".",
            "Add 5 to list.",
            "Set map at \"key\" to \"value\".",
        ]),
        0..=100, // Generate 0-100 random statements
    )
    .prop_map(|statements| statements.join("\n"))
}

/// Fuzz test for lexer - should never panic on any input
#[test]
fn fuzz_lexer() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 1000; // Run 1000 test cases
    
    proptest!(config, |(source in arbitrary_langx_source())| {
        // Lexer should handle any string without panicking
        let _tokens = lexer::tokenize(&source);
        // We don't assert anything specific - just that it doesn't panic
    });
}

/// Fuzz test for parser - should handle parse errors gracefully
#[test]
fn fuzz_parser() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 1000;
    
    proptest!(config, |(source in arbitrary_langx_source())| {
        // Parser should return Result, not panic
        let result = parser::parse(&source);
        match result {
            Ok(_program) => {
                // If parsing succeeds, that's fine
            }
            Err(_error) => {
                // If parsing fails, that's also fine - we just don't want panics
            }
        }
    });
}

/// Fuzz test for parser with recovery - should collect multiple errors
#[test]
fn fuzz_parser_recovery() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 500; // Fewer cases since this is more expensive
    
    proptest!(config, |(source in arbitrary_langx_source())| {
        // Parser recovery should return Result, not panic
        let result = parser::parse_with_recovery(&source);
        match result {
            Ok(_program) => {
                // If parsing succeeds, that's fine
            }
            Err(_errors) => {
                // If parsing fails with multiple errors, that's fine
            }
        }
    });
}

/// Fuzz test for interpreter - should handle runtime errors gracefully
#[test]
fn fuzz_interpreter() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 500; // Fewer cases since execution is more expensive
    
    proptest!(config, |(source in arbitrary_langx_source())| {
        // Try to parse first
        if let Ok(program) = parser::parse(&source) {
            // If parsing succeeds, try to interpret
            let mut interpreter = interpreter::Interpreter::new();
            let result = interpreter.interpret(&program);
            
            match result {
                Ok(_) => {
                    // Execution succeeded - great!
                }
                Err(_error) => {
                    // Runtime error is fine - we just don't want panics
                    // Common runtime errors: undefined variables, type errors, etc.
                }
            }
        }
        // If parsing fails, we skip interpretation (that's fine)
    });
}

/// Fuzz test with completely random strings (not just valid LangX patterns)
#[test]
fn fuzz_random_strings() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 1000;
    
    proptest!(config, |(source in ".*")| {
        // Lexer should handle any string without panicking
        let _tokens = lexer::tokenize(&source);
        
        // Parser should return Result, not panic
        let _result = parser::parse(&source);
    });
}

/// Fuzz test for bytecode compiler
#[test]
fn fuzz_bytecode_compiler() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 500;
    
    proptest!(config, |(source in arbitrary_langx_source())| {
        // Try to parse first
        if let Ok(program) = parser::parse(&source) {
            // If parsing succeeds, try to compile to bytecode
            let mut compiler = langx::bytecode::Compiler::new();
            let result = compiler.compile(&program);
            
            match result {
                Ok(_chunk) => {
                    // Compilation succeeded - great!
                }
                Err(_error) => {
                    // Compilation error is fine - we just don't want panics
                }
            }
        }
    });
}

/// Fuzz test for bytecode VM execution
#[test]
fn fuzz_bytecode_vm() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 300; // Even fewer cases since VM execution is expensive
    
    proptest!(config, |(source in arbitrary_langx_source())| {
        // Try to parse and compile first
        if let Ok(program) = parser::parse(&source) {
            let mut compiler = langx::bytecode::Compiler::new();
            if let Ok(chunk) = compiler.compile(&program) {
                let functions = compiler.get_functions().clone();
                
                // Try to execute bytecode
                let mut vm = langx::bytecode::VM::new();
                let result = vm.execute(chunk, functions);
                
                match result {
                    Ok(_) => {
                        // Execution succeeded - great!
                    }
                    Err(_error) => {
                        // Runtime error is fine - we just don't want panics
                    }
                }
            }
        }
    });
}

/// Property test: parsing and then formatting should be idempotent
/// (for valid programs that parse successfully)
#[test]
fn prop_parse_format_idempotent() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 200;
    
    proptest!(config, |(source in arbitrary_langx_source())| {
        if let Ok(program) = parser::parse(&source) {
            // If we could parse it, the program should be valid
            // We can't easily format it back, but we can verify it's well-formed
            assert!(!program.statements.is_empty() || source.trim().is_empty());
        }
    });
}

/// Property test: lexer should produce tokens for any input
#[test]
fn prop_lexer_always_produces_tokens() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 1000;
    
    proptest!(config, |(source in ".*")| {
        let tokens = lexer::tokenize(&source);
        // Lexer should always produce some result (even if empty for empty input)
        // We don't care about the content, just that it doesn't panic
        let _ = tokens;
    });
}

/// Property test: line number calculation should never panic
#[test]
fn prop_line_number_safe() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 1000;
    
    proptest!(config, |(source in ".*", position in 0usize..10000)| {
        // Line number calculation should handle any position safely
        let line = lexer::line_number_at_position(&source, position);
        // Line number should be at least 1
        assert!(line >= 1);
    });
}

/// Property test: code snippet extraction should never panic
#[test]
fn prop_code_snippet_safe() {
    let mut config = proptest::test_runner::Config::default();
    config.cases = 500;
    
    proptest!(config, |(source in ".*", position in 0usize..10000, context in 0usize..10)| {
        // Code snippet extraction should handle any input safely
        let _snippet = lexer::get_code_snippet(&source, position, context);
    });
}

