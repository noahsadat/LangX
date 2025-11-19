# LangX VS Code Extension

Syntax highlighting and language support for the LangX programming language.

## Features

- ✅ Syntax highlighting for LangX code
- ✅ Comment support (`#` for line comments)
- ✅ String highlighting (single and triple-quoted strings)
- ✅ Keyword highlighting (control flow, operators, built-in functions)
- ✅ Number and identifier highlighting
- ✅ Bracket matching and auto-closing
- ✅ Code folding for blocks (Repeat, While, For, Define, If)
- ✅ Smart indentation

## Installation

### Option 1: Install from VSIX (Production)

1. Package the extension:
   ```bash
   cd vscode-langx
   vsce package
   ```

2. Install in Cursor/VS Code:
   - Press `Cmd+Shift+P` (Mac) or `Ctrl+Shift+P` (Windows/Linux)
   - Type: `Extensions: Install from VSIX...`
   - Select `langx-0.1.0.vsix`
   - Reload the window

### Option 2: Development Mode (No Packaging Needed)

For development and testing, you can run the extension directly from source:

1. Open the `vscode-langx` folder in Cursor/VS Code
2. Press `F5` (or go to Run → Start Debugging)
3. A new window will open with the extension loaded
4. Open any `.lx` file to see syntax highlighting

**Note:** This is a pure syntax highlighting extension (no code), so it works immediately without any build step!

## Language Features

The extension provides:

- **Syntax Highlighting**: Colors keywords, strings, numbers, comments, and operators
- **Bracket Matching**: Automatically matches brackets, braces, and parentheses
- **Code Folding**: Fold blocks defined by `Repeat`, `While`, `For`, `Define`, and `If`
- **Smart Indentation**: Automatically indents code blocks

## Supported File Extensions

- `.lx` - LangX source files

## Contributing

Contributions are welcome! Please see the main LangX repository for contribution guidelines.

## License

MIT License - see LICENSE file in the main repository.

