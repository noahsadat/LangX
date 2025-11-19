mod repl;
#[cfg(test)]
mod tests;

use langx::{parser, interpreter};

use std::env;
use std::fs;
use rustyline::error::ReadlineError;
use rustyline::{Config, Editor};
use repl::LangXHelper;

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
    
    // Create REPL helper with completion, highlighting, and hints
    let helper = LangXHelper {
        completer: repl::LangXCompleter::new(),
        highlighter: repl::LangXHighlighter::new(),
        hinter: rustyline::hint::HistoryHinter {},
        validator: rustyline::validate::MatchingBracketValidator::new(),
    };
    
    let config = Config::builder()
        .auto_add_history(true)
        .completion_type(rustyline::CompletionType::List)
        .build();
    
    let mut rl = Editor::with_config(config)
        .map_err(|e| format!("Failed to initialize REPL: {}", e))?;
    rl.set_helper(Some(helper));
    
    println!("LangX REPL - Type 'exit' to quit");
    println!("Features:");
    println!("  - Arrow keys: Navigate command history");
    println!("  - Tab: Auto-complete keywords and functions");
    println!("  - Syntax highlighting enabled");
    println!("  - Type 'debug' to start debugger mode");
    
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }
                
                let trimmed = line.trim();
                if trimmed == "exit" {
                    break;
                }
                
                // Check for debugger command
                if trimmed == "debug" {
                    println!("Starting debugger mode...");
                    println!("Commands: step, continue, inspect <var>, breakpoint <line>, quit");
                    if let Err(e) = run_debugger(&mut interpreter) {
                        eprintln!("Debugger error: {}", e);
                    }
                    continue;
                }
                
                match parser::parse(&line) {
                    Ok(program) => {
                        if let Err(e) = interpreter.interpret(&program) {
                            eprintln!("\x1b[1;31mRuntime error:\x1b[0m {}", e);
                        }
                    }
                    Err(e) => eprintln!("\x1b[1;31mParse error:\x1b[0m {}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                return Err(format!("REPL error: {}", err));
            }
        }
    }
    
    Ok(())
}

fn run_debugger(interpreter: &mut interpreter::Interpreter) -> Result<(), String> {
    use rustyline::DefaultEditor;
    use rustyline::error::ReadlineError;
    
    let mut rl = DefaultEditor::new()
        .map_err(|e| format!("Failed to initialize debugger: {}", e))?;
    
    println!("Debugger ready. Type 'help' for commands.");
    
    loop {
        let readline = rl.readline("(debug) ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                match trimmed {
                    "quit" | "exit" => {
                        println!("Exiting debugger.");
                        break;
                    }
                    "help" => {
                        println!("Debugger commands:");
                        println!("  inspect <var> - Inspect variable value");
                        println!("  vars - List all variables with values");
                        println!("  funcs - List all user-defined functions");
                        println!("  quit - Exit debugger");
                    }
                    "vars" => {
                        let vars = interpreter.list_variables();
                        if vars.is_empty() {
                            println!("No variables defined.");
                        } else {
                            println!("Variables:");
                            for var in vars {
                                if let Some(value) = interpreter.get_variable(&var) {
                                    println!("  {} = {}", var, value);
                                }
                            }
                        }
                    }
                    "funcs" => {
                        let funcs = interpreter.list_functions();
                        if funcs.is_empty() {
                            println!("No user-defined functions.");
                        } else {
                            println!("Functions:");
                            for func in funcs {
                                println!("  {}", func);
                            }
                        }
                    }
                    cmd if cmd.starts_with("inspect ") => {
                        let var_name = cmd.strip_prefix("inspect ").unwrap_or("");
                        if var_name.is_empty() {
                            println!("Usage: inspect <variable_name>");
                        } else {
                            match interpreter.get_variable(var_name) {
                                Some(value) => {
                                    println!("{} = {}", var_name, value);
                                }
                                None => {
                                    println!("Variable '{}' not found.", var_name);
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Unknown command. Type 'help' for available commands.");
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                return Err(format!("Debugger error: {}", err));
            }
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
