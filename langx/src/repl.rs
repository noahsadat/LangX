use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Context, Helper};
use rustyline::Result as RustylineResult;
use std::borrow::Cow::{self, Borrowed, Owned};

/// LangX keywords for auto-completion
const KEYWORDS: &[&str] = &[
    "Set", "to", "If", "then", "is", "greater", "than", "less", "equal", "not",
    "print", "Repeat", "times", "While", "End", "while", "For", "each", "in",
    "End for", "Define", "with", "parameters", "parameter", "End definition",
    "Call", "Return", "Break", "loop", "Continue", "to next iteration",
    "Add", "item", "of", "and", "or", "true", "false",
];

/// Built-in functions for auto-completion
const BUILTIN_FUNCTIONS: &[&str] = &[
    "string_length", "substring", "split", "join", "replace",
    "abs", "min", "max", "pow", "sqrt", "round", "floor", "ceil",
    "read_file", "write_file",
    "current_timestamp", "current_datetime", "format_timestamp", "time_difference",
];

/// Helper struct for REPL features (completion, highlighting, hints)
pub struct LangXHelper {
    pub completer: LangXCompleter,
    pub highlighter: LangXHighlighter,
    pub hinter: HistoryHinter,
    pub validator: MatchingBracketValidator,
}

impl Helper for LangXHelper {}

impl Completer for LangXHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> RustylineResult<(usize, Vec<Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for LangXHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for LangXHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(prompt)
        } else {
            Owned(format!("\x1b[1;32m{}\x1b[0m", prompt))
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[1;30m{}\x1b[0m", hint))
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for LangXHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> RustylineResult<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

/// Completer for LangX keywords and built-in functions
pub struct LangXCompleter {
    keywords: Vec<String>,
    builtin_functions: Vec<String>,
}

impl LangXCompleter {
    pub fn new() -> Self {
        Self {
            keywords: KEYWORDS.iter().map(|s| s.to_string()).collect(),
            builtin_functions: BUILTIN_FUNCTIONS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn add_variable(&mut self, _name: String) {
        // Variables are added dynamically during REPL session
        // For now, we'll track them in the completer
    }

    pub fn add_function(&mut self, _name: String) {
        // User-defined functions can be added here
    }
}

impl Completer for LangXCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> RustylineResult<(usize, Vec<Pair>)> {
        let start = line[..pos]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);

        let word = &line[start..pos];
        let mut candidates = Vec::new();

        // Add keywords
        for keyword in &self.keywords {
            if keyword.starts_with(word) && keyword != word {
                candidates.push(Pair {
                    display: keyword.clone(),
                    replacement: keyword.clone(),
                });
            }
        }

        // Add built-in functions
        for func in &self.builtin_functions {
            if func.starts_with(word) && func != word {
                candidates.push(Pair {
                    display: func.clone(),
                    replacement: func.clone(),
                });
            }
        }

        Ok((start, candidates))
    }
}

/// Highlighter for syntax highlighting in REPL
pub struct LangXHighlighter;

impl LangXHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for LangXHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        // Simple keyword highlighting
        let mut result = line.to_string();
        
        // Highlight keywords (in reverse order to avoid partial matches)
        let mut sorted_keywords: Vec<&str> = KEYWORDS.iter().copied().collect();
        sorted_keywords.sort_by(|a, b| b.len().cmp(&a.len())); // Sort by length descending
        
        for keyword in sorted_keywords {
            let replacement = format!("\x1b[1;34m{}\x1b[0m", keyword); // Blue
            result = result.replace(keyword, &replacement);
        }

        // Highlight built-in functions
        for func in BUILTIN_FUNCTIONS {
            let replacement = format!("\x1b[1;33m{}\x1b[0m", func); // Yellow
            result = result.replace(func, &replacement);
        }

        // Highlight numbers (simple approach)
        let mut highlighted = String::new();
        let mut chars = result.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch.is_ascii_digit() {
                let mut num = String::from(ch);
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        num.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                highlighted.push_str(&format!("\x1b[1;36m{}\x1b[0m", num)); // Cyan
            } else {
                highlighted.push(ch);
            }
        }

        Owned(highlighted)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        if pos >= line.len() {
            return false;
        }
        let ch = line.chars().nth(pos).unwrap();
        ch == '(' || ch == ')' || ch == '[' || ch == ']' || ch == '{' || ch == '}'
    }
}

