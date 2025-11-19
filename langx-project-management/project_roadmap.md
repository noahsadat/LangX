# LangX Project Roadmap

## Project Overview
LangX is a programming language that combines structured English with mathematical symbols to create executable code. Built entirely in Rust, it parses and executes hybrid-syntax statements deterministically without relying on AI or machine learning at runtime.

## Core Components

### 1. Language Design [COMPLETE]
- [x] Core language syntax definition
- [x] Grammar specification (LALRPOP)
- [x] Hybrid syntax design (symbols + words)
- [x] Operator precedence rules
- [x] Error handling conventions
- [ ] Extended language features
- [ ] Standard library functions

### 2. Lexer [COMPLETE]
- [x] Token definitions
- [x] Lexical analyzer using Logos
- [x] Symbol tokens (+, -, *, /)
- [x] Word tokens (and, or, not, If, then, etc.)
- [x] Comment support (#)
- [x] Boolean literals (true, false)
- [x] Comprehensive tokenization tests

### 3. Parser [COMPLETE]
- [x] LALRPOP grammar definition
- [x] AST structure design
- [x] Parser implementation
- [x] **Zero ambiguity warnings** (LR(1) compatible)
- [x] Operator precedence handling
- [x] Parentheses support
- [x] Comprehensive parsing tests

### 4. Interpreter [COMPLETE]
- [x] Variable environment
- [x] Expression evaluation
- [x] Statement execution
- [x] Control flow (If...then, Repeat, While)
- [x] Functions and procedures (0-5 params) - **Extended from 0-2**
- [x] Return value handling
- [x] Proper scoping rules
- [x] Error messages and handling
- [x] **Enhanced error messages with line numbers** - ✅ December 2025
- [x] **String concatenation** - ✅ December 2025
- [x] **Built-in functions** - ✅ December 2025
- [ ] Advanced error recovery

### 5. CLI & REPL [COMPLETE]
- [x] Interactive REPL implementation
- [x] File execution support (.lx files)
- [x] Command-line interface
- [ ] Command history (arrow keys)
- [ ] Auto-completion
- [ ] Syntax highlighting
- [ ] Interactive debugger

### 6. Standard Library [IN PROGRESS]
- [ ] Math functions (sqrt, pow, abs, etc.)
- [x] **String manipulation** - ✅ Started December 2025
  - [x] `string_length` - Get length of a string
  - [x] `substring` - Extract substring (string, start, length)
  - [x] String concatenation with `+` operator
- [ ] More string functions (split, join, replace)
- [ ] I/O operations (file read/write)
- [x] **Data structures** - ✅ Lists implemented
- [ ] Time and date handling

### 7. Advanced Features [NOT STARTED]
- [ ] Module system
- [ ] Package management
- [ ] Bytecode compiler
- [ ] WebAssembly target
- [ ] Performance optimizations
- [ ] JIT compilation

### 8. Documentation [COMPLETE]
- [x] Comprehensive README
- [x] Language syntax reference
- [x] Updated project descriptions
- [x] Implementation plans
- [x] Example programs (15+ examples)
- [ ] API documentation
- [ ] Tutorial series
- [ ] Video tutorials

### 9. Testing [IN PROGRESS]
- [x] Unit tests for core components
- [x] Integration tests
- [x] Example programs as tests
- [x] **Comprehensive test suite** - ✅ Added 20+ new tests December 2025
  - [x] String concatenation tests (4 tests)
  - [x] Multi-parameter function tests (3 tests)
  - [x] Operator precedence tests
  - [x] Error handling tests
  - [x] Edge case tests
- [ ] Benchmarks
- [ ] Fuzzing tests
- [ ] Code coverage reports (aim for 90%+) - **Current: ~70%**

### 10. Distribution [NOT STARTED]
- [ ] Release packaging
- [ ] Installation scripts (brew, apt, etc.)
- [ ] Online playground/REPL
- [ ] VS Code extension
- [ ] IDE integrations

## Major Milestones

### ✅ Milestone 1: Core Language (October 2025)
- [x] Complete interpreter with all basic features
- [x] Hybrid syntax (symbols for math, words for logic)
- [x] Function support with parameters and returns
- [x] Arithmetic operations with proper precedence
- [x] Boolean logic and comparisons
- [x] Zero parser ambiguities
- [x] Comprehensive example programs
- [x] Updated documentation

**Achievement:** LangX is now a fully functional interpreted language!

### 🎯 Milestone 2: Enhanced Features (Q1 2026) [IN PROGRESS]
- [x] **Extended function support** - ✅ Now supports 0-5 parameters (December 2025)
- [x] **Advanced control flow** - ✅ While loops implemented
- [x] **List/array data structures** - ✅ Implemented
- [x] **String manipulation functions** - ✅ Started (length, substring, concatenation)
- [ ] File I/O operations
- [x] **Improved error messages** - ✅ Line numbers and code snippets (December 2025)
- [ ] Variadic function arguments
- [ ] Default function parameters
- [ ] For loops

### 🎯 Milestone 3: Performance & Tooling (Q2 2026)
- [ ] Bytecode compiler
- [ ] Performance benchmarks
- [ ] VS Code extension
- [ ] Online playground
- [ ] Comprehensive test suite (90%+ coverage)

### 🎯 Milestone 4: Distribution (Q3 2026)
- [ ] Package manager integration
- [ ] Multi-platform binaries
- [ ] Standard library v1.0
- [ ] Official documentation site
- [ ] Community examples repository

## Current Status (December 2025)

### What's Working
✅ **Fully functional interpreter**
✅ **Variables and assignments**
✅ **Arithmetic** with symbols: `+`, `-`, `*`, `/`
✅ **Logic** with words: `and`, `or`, `not`
✅ **Booleans**: `true`, `false`
✅ **Comparisons**: `is greater than`, `is less than`, `is equal to`, `is not equal to`
✅ **Conditionals**: `If...then`
✅ **Loops**: `Repeat...times`, `While...End while`
✅ **Functions**: 0-5 parameters with return values - **Extended from 0-2**
✅ **Comments**: `#` prefix
✅ **Operator precedence**: Correct mathematical precedence
✅ **Parentheses**: Expression grouping (fixed December 2025)
✅ **REPL**: Interactive mode
✅ **File execution**: Run `.lx` files
✅ **String concatenation**: `"text" + 42`, `100 + " percent"`, `"Status: " + true`
✅ **Lists/Arrays**: Creation, indexing, appending
✅ **Built-in functions**: `string_length`, `substring`
✅ **Enhanced error messages**: Line numbers and code snippets

### Key Design Decisions
1. **Hybrid Syntax** - Symbols for math, words for logic
2. **Comma Separators** - Function args use `,` not `and`
3. **"then" Keyword** - If statements use `then` not `,`
4. **Zero Ambiguity** - Clean LR(1) grammar with no conflicts

## Technical Achievements
- ✅ **Zero LALRPOP warnings** - Clean grammar
- ✅ **Proper operator precedence** - Mathematical rules
- ✅ **Solved parser ambiguity** - Hybrid syntax approach
- ✅ **Functional scoping** - Nested environments work correctly
- ✅ **Type safety** - Runtime type checking

## Next Focus Areas

### Immediate (Next 2 weeks)
1. ~~Add while loops~~ - ✅ **Completed**
2. ~~Implement lists/arrays~~ - ✅ **Completed**
3. ~~String manipulation functions~~ - ✅ **Partially completed** (length, substring, concat)
4. ~~Enhanced error messages~~ - ✅ **Completed**
5. Add more string functions (split, join, replace)
6. Improve test coverage to 90%+

### Short-term (Next 2 months)
1. Standard library core functions
2. File I/O operations
3. VS Code syntax highlighting
4. Online REPL/playground

### Long-term (6-12 months)
1. Bytecode compilation
2. WebAssembly target
3. Package manager
4. Performance optimizations

## Long-term Vision
LangX aims to prove that programming languages can be both:
- **Readable** like English
- **Precise** like mathematics

By combining the best of natural language with symbolic precision, LangX creates a unique programming experience that is accessible to beginners yet powerful for experts.

## Community & Contribution
- [ ] Contribution guidelines
- [ ] Code of conduct
- [ ] Example submissions
- [ ] Language feature proposals
- [ ] Discord/Slack community
- [ ] Regular dev updates

## Success Metrics
- Interpreter stability: **Stable** ✅
- Parser correctness: **100%** ✅
- Test coverage: **~70%** (target: 90%) - **Improved from 60%**
- Documentation: **Complete** ✅
- Community adoption: **Just starting**

## Recent Updates (December 2025)
- ✅ Enhanced error messages with line numbers
- ✅ Extended function support to 5 parameters
- ✅ String concatenation with multiple types
- ✅ Built-in string functions (`string_length`, `substring`)
- ✅ Fixed parentheses precedence
- ✅ Comprehensive test suite (20+ new tests)
- ✅ Fixed all example file syntax issues

---

**Last Updated:** December 2025
**Current Version:** 0.2.0
**Status:** Milestone 2 In Progress (40% complete)
