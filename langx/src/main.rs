mod lexer;
mod parser;
mod ast;
mod interpreter;
#[cfg(test)]
mod tests;

use std::env;
use std::fs;
use std::io::{self, Write};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // Run a file
        let filename = &args[1];
        run_file(filename)
    } else {
        // Start REPL
        run_repl()
    }
}

fn run_file(filename: &str) -> Result<(), String> {
    let source = fs::read_to_string(filename)
        .map_err(|e| format!("Error reading file: {}", e))?;
    
    run(&source)
}

fn run_repl() -> Result<(), String> {
    let mut interpreter = interpreter::Interpreter::new();
    
    println!("LangX REPL - Type 'exit' to quit");
    
    loop {
        print!("> ");
        io::stdout().flush().map_err(|e| format!("IO error: {}", e))?;
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Error reading input: {}", e))?;
        
        if input.trim().is_empty() {
            continue;
        }
        
        if input.trim() == "exit" {
            break;
        }
        
        match parser::parse(&input) {
            Ok(program) => {
                if let Err(e) = interpreter.interpret(&program) {
                    eprintln!("Runtime error: {}", e);
                }
            }
            Err(e) => eprintln!("Parse error: {}", e),
        }
    }
    
    Ok(())
}

fn run(source: &str) -> Result<(), String> {
    // Parse the source code
    let program = parser::parse(source)?;
    
    // Interpret the program
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(&program)?;
    
    Ok(())
}
