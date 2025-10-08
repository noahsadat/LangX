use std::fmt;

/// The AST node types for LangX
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
}

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

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
}

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
    
    /// Block of statements
    Block(Vec<Statement>),
    
    /// Function definition (e.g., Define add with parameters a, b: Return a plus b. End definition.)
    FunctionDefinition {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    
    /// Return statement (e.g., Return x)
    Return(Option<Expression>),
}

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
            Statement::Block(statements) => {
                write!(f, "{{")?;
                for stmt in statements {
                    write!(f, " {};", stmt)?;
                }
                write!(f, " }}")
            },
            Statement::FunctionDefinition { name, parameters, body } => {
                write!(f, "Define {} with parameters {}:\n", name, parameters.join(", "))?;
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