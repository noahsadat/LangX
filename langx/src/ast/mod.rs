//! # Abstract Syntax Tree (AST) Module
//!
//! This module defines the Abstract Syntax Tree representation of LangX programs.
//! The AST is the output of the parser and the input to both the interpreter
//! and bytecode compiler.
//!
//! ## Structure
//!
//! The AST consists of:
//! - **Expressions**: Values, operations, function calls, data structure access
//! - **Statements**: Assignments, conditionals, loops, function definitions, prints
//! - **Program**: Top-level container for a complete LangX program
//!
//! ## Example
//!
//! ```rust
//! use langx::ast::{Program, Statement, Expression};
//!
//! // A simple program: Set x to 10. print x.
//! let program = Program {
//!     statements: vec![
//!         Statement::Assignment {
//!             variable: "x".to_string(),
//!             value: Expression::Number(10),
//!         },
//!         Statement::Print(Expression::Variable("x".to_string())),
//!     ],
//! };
//! ```

use std::fmt;

/// A function parameter with optional default value and variadic flag.
///
/// Parameters can be:
/// - Regular parameters: `name`
/// - Parameters with default values: `name default value`
/// - Variadic parameters: `...name` (accepts variable number of arguments)
///
/// # Fields
///
/// * `name` - The parameter name
/// * `default_value` - Optional default value expression
/// * `is_variadic` - Whether this parameter accepts variable arguments
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Expression>,
    pub is_variadic: bool,
}

impl Parameter {
    pub fn new(name: String) -> Self {
        Self {
            name,
            default_value: None,
            is_variadic: false,
        }
    }
    
    pub fn with_default(name: String, default_value: Expression) -> Self {
        Self {
            name,
            default_value: Some(default_value),
            is_variadic: false,
        }
    }
    
    pub fn variadic(name: String) -> Self {
        Self {
            name,
            default_value: None,
            is_variadic: true,
        }
    }
}

/// Expression nodes in the LangX AST.
///
/// Expressions represent values and operations that can be evaluated to produce a value.
/// This includes literals, variables, binary/unary operations, function calls, and
/// data structure access.
///
/// # Variants
///
/// - **Literals**: `Number`, `String`, `Boolean`
/// - **Variables**: `Variable(name)`
/// - **Operations**: `BinaryOp`, `UnaryOp`
/// - **Function Calls**: `FunctionCall`
/// - **Data Structures**: `List`, `ListIndex`, `Map`, `MapIndex`
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// A numeric literal (e.g., 42)
    Number(i64),
    
    /// A string literal (e.g., "Hello")
    String(String),
    
    /// A boolean literal (e.g., true, false)
    Boolean(bool),
    
    /// A variable reference (e.g., x)
    Variable(String),
    
    /// A binary operation (e.g., x > 5)
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    
    /// A unary operation (e.g., not x)
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    
    /// A function call (e.g., Call add with 1, 2)
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    
    /// A list literal (e.g., [1, 2, 3])
    List(Vec<Expression>),
    
    /// List indexing (e.g., item 0 of list)
    ListIndex {
        list: Box<Expression>,
        index: Box<Expression>,
    },
    
    /// A map literal (e.g., {"key": value, "key2": value2})
    Map(Vec<(Expression, Expression)>),
    
    /// Map indexing (e.g., map at "key")
    MapIndex {
        map: Box<Expression>,
        key: Box<Expression>,
    },
}

/// Binary operators in LangX expressions.
///
/// These operators combine two expressions to produce a result.
/// Includes arithmetic, comparison, and logical operators.
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    Plus,
    Minus,
    Times,
    Divide,
    And,
    Or,
}

/// Unary operators in LangX expressions.
///
/// These operators operate on a single expression.
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    /// Logical negation: `not x`
    Not,
}

/// Statement nodes in the LangX AST.
///
/// Statements represent actions and control flow constructs that don't necessarily
/// produce values (unlike expressions). This includes assignments, conditionals,
/// loops, function definitions, and prints.
///
/// # Variants
///
/// - **Assignments**: `Assignment`, `MapAssignment`, `ListAppend`
/// - **Control Flow**: `Conditional`, `Repeat`, `While`, `For`, `Break`, `Continue`
/// - **Functions**: `FunctionDefinition`, `Return`
/// - **I/O**: `Print`
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// Assignment statement (e.g., Set x to 10)
    Assignment {
        variable: String,
        value: Expression,
    },
    
    /// Conditional statement (e.g., If x is greater than 5, print "Hello")
    Conditional {
        condition: Expression,
        then_branch: Box<Statement>,
    },
    
    /// Print statement (e.g., print "Hello")
    Print(Expression),
    
    /// Repeat statement (e.g., Repeat 3 times: print "Hello")
    Repeat {
        count: Expression,
        body: Box<Statement>,
    },
    
    /// While statement (e.g., While condition: statement)
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    
    /// For loop statement (e.g., For each item in list: statement)
    For {
        variable: String,
        list: Expression,
        body: Box<Statement>,
    },
    
    /// Block of statements
    Block(Vec<Statement>),
    
    /// Function definition (e.g., Define add with parameters a, b: Return a plus b. End definition.)
    FunctionDefinition {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
    },
    
    /// Return statement (e.g., Return x)
    Return(Option<Expression>),
    
    /// List append statement (e.g., Add 6 to list)
    ListAppend {
        list_name: String,
        value: Expression,
    },
    
    /// Map assignment statement (e.g., Set map at "key" to value)
    MapAssignment {
        map_name: String,
        key: Expression,
        value: Expression,
    },
    
    /// Break statement (e.g., Break loop.)
    Break,
    
    /// Continue statement (e.g., Continue to next iteration.)
    Continue,
}

/// A complete LangX program.
///
/// This is the top-level AST node representing an entire LangX program.
/// It contains a sequence of statements that are executed in order.
///
/// # Fields
///
/// * `statements` - Vector of statements that make up the program
#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::Boolean(b) => write!(f, "{}", b),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::BinaryOp { left, operator, right } => {
                write!(f, "({} {} {})", left, operator, right)
            },
            Expression::UnaryOp { operator, operand } => {
                write!(f, "{} {}", operator, operand)
            },
            Expression::FunctionCall { name, arguments } => {
                write!(f, "Call {} with ", name)?;
                let args: Vec<String> = arguments.iter()
                    .map(|arg| format!("{}", arg))
                    .collect();
                write!(f, "{}", args.join(", "))
            }
            Expression::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Expression::ListIndex { list, index } => {
                write!(f, "item {} of {}", index, list)
            }
            Expression::Map(entries) => {
                write!(f, "{{")?;
                for (i, (key, value)) in entries.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
            Expression::MapIndex { map, key } => {
                write!(f, "{} at {}", map, key)
            }
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::GreaterThan => write!(f, "greater than"),
            BinaryOperator::LessThan => write!(f, "less than"),
            BinaryOperator::Equal => write!(f, "equal to"),
            BinaryOperator::NotEqual => write!(f, "not equal to"),
            BinaryOperator::Plus => write!(f, "plus"),
            BinaryOperator::Minus => write!(f, "minus"),
            BinaryOperator::Times => write!(f, "times"),
            BinaryOperator::Divide => write!(f, "divided by"),
            BinaryOperator::And => write!(f, "and"),
            BinaryOperator::Or => write!(f, "or"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "not"),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Assignment { variable, value } => {
                write!(f, "Set {} to {}", variable, value)
            }
            Statement::Conditional { condition, then_branch } => {
                write!(f, "If {}, {}", condition, then_branch)
            }
            Statement::Print(expr) => {
                write!(f, "print {}", expr)
            }
            Statement::Repeat { count, body } => {
                write!(f, "Repeat {} times: {}", count, body)
            }
            Statement::While { condition, body } => {
                write!(f, "While {}: {}", condition, body)
            }
            Statement::For { variable, list, body } => {
                write!(f, "For each {} in {}: {}", variable, list, body)
            }
            Statement::Block(statements) => {
                write!(f, "{{")?;
                for stmt in statements {
                    write!(f, " {};", stmt)?;
                }
                write!(f, " }}")
            },
            Statement::FunctionDefinition { name, parameters, body } => {
                write!(f, "Define {} with parameters ", name)?;
                let param_strs: Vec<String> = parameters.iter().map(|p| {
                    let mut s = if p.is_variadic { format!("...{}", p.name) } else { p.name.clone() };
                    if let Some(ref default) = p.default_value {
                        s.push_str(&format!(" default {}", default));
                    }
                    s
                }).collect();
                write!(f, "{}:\n", param_strs.join(", "))?;
                for stmt in body {
                    write!(f, "    {}\n", stmt)?;
                }
                write!(f, "End definition")
            },
            Statement::Return(expr) => {
                match expr {
                    Some(e) => write!(f, "Return {}", e),
                    None => write!(f, "Return"),
                }
            }
            Statement::ListAppend { list_name, value } => {
                write!(f, "Add {} to {}", value, list_name)
            }
            Statement::MapAssignment { map_name, key, value } => {
                write!(f, "Set {} at {} to {}", map_name, key, value)
            }
            Statement::Break => {
                write!(f, "Break loop")
            }
            Statement::Continue => {
                write!(f, "Continue to next iteration")
            }
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
} 