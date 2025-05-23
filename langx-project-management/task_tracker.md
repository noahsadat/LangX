# LangX Task Tracker

## Current Sprint: Operator Precedence & Complex Expressions

### Completed Tasks
- [x] Set up basic project structure
- [x] Implement lexer with Logos
- [x] Define grammar with LALRPOP
- [x] Create AST structure
- [x] Implement basic interpreter
- [x] Add variable assignment support
- [x] Add conditional statements
- [x] Add repeat loops
- [x] Implement print statements
- [x] Create basic REPL
- [x] Implement function definitions and calls
- [x] Add support for function parameters
- [x] Add return statement support
- [x] Implement scoping for functions
- [x] Implement addition operation
- [x] Implement subtraction operation
- [x] Implement multiplication operation
- [x] Implement division operation

### In Progress
- [ ] Implement operator precedence
  - [ ] Update grammar structure for precedence
  - [ ] Add tests for operator precedence
  - [ ] Update example programs to demonstrate precedence
- [ ] Add support for parentheses in expressions
- [ ] Improve error messages and error handling

### Upcoming Tasks
- [ ] Add string concatenation
- [ ] Implement comparison operators (less than, equal to, not equal to)
- [ ] Implement logical operators (and, or, not)
- [ ] Add more test cases and examples
- [ ] Improve REPL with command history and auto-completion

## Technical Debt
- Refactor interpreter to use visitor pattern
- Add more comprehensive error handling
- Improve test coverage for edge cases

## Known Issues
1. No proper error recovery in parser
2. Limited expression types (only numbers, strings, and variables)
3. Arithmetic operations don't respect operator precedence yet
4. Missing standard library functions

## Next Features to Implement
1. Operator precedence
2. Boolean expressions and logical operators
3. String concatenation and manipulation
4. Lists and basic data structures
5. File I/O operations

## Development Workflow
1. Feature planning and specification
2. Implementation
3. Unit testing
4. Documentation
5. Code review
6. Integration testing
7. Release

## Meeting Notes
Last meeting (2023-08-29):
- Completed all basic arithmetic operations
- Discussed approach for implementing operator precedence
- Assigned tasks for next sprint

Next meeting: 2023-09-05
- Review progress on operator precedence
- Plan next sprint tasks
- Discuss boolean operations 