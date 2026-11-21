# bwh-rust

A lexer, parser, and REPL for the "badwhipp" language, written in Rust, inspired by the "Writing an Interpreter in Go" book.

## Overview

This project implements a lexical analyzer (lexer) that converts source code into tokens, a parser that builds an Abstract Syntax Tree (AST), and an interactive REPL (Read-Eval-Print Loop) for experimenting with the language. It's a learning project demonstrating fundamental compiler concepts in Rust.

## Features

- **Token Types**: Supports keywords (`LET`, `FUNCTION`, `IF`, `ELSE`, `TRUE`, `FALSE`, `RETURN`), identifiers (`IDENT`), integers (`INT`), operators (`+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `>`, `!`), punctuation (`;`, `,`, `(`, `)`, `{`, `}`), and special tokens (`ILLEGAL`, `EOF`)
- **Lexer**: Character-by-character scanning with position tracking
- **Parser**: Recursive descent parser that builds an AST with let statement support
- **AST**: Abstract Syntax Tree nodes for statements and expressions
- **REPL**: Interactive command-line interface for tokenizing input in real-time
- **Tests**: Unit tests for lexer (passing) and parser (failing - test input needs fixing)

## Project Structure

```
src/
├── lib.rs      # Library entry point, exports all modules
├── main.rs     # Binary entry point (starts REPL)
├── token.rs    # Token definitions and types
├── lexer.rs    # Lexer implementation with tests
├── ast.rs      # Abstract Syntax Tree node definitions
├── parser.rs   # Parser implementation with let statement parsing
└── repl.rs     # REPL implementation
```

## Building

```bash
cargo build
```

## Running Tests

```bash
cargo test
```

Currently 1 test passing (lexer), 1 test failing (parser - test input has malformed let statements).

## Running the REPL

The binary starts an interactive REPL:

```bash
cargo run
```

### Example REPL Session

```
$ cargo run
Hello! This is the badwhipp language implemented in rust
Feel free to type in commands
>> let x = 5;
Token { token_type: "LET", token_literal: "let" }
Token { token_type: "IDENT", token_literal: "x" }
Token { token_type: "=", token_literal: "=" }
Token { token_type: "INT", token_literal: "5" }
Token { token_type: ";", token_literal: ";" }
>> fn add(a, b) { return a + b; }
Token { token_type: "FUNCTION", token_literal: "fn" }
Token { token_type: "IDENT", token_literal: "add" }
Token { token_type: "(", token_literal: "(" }
Token { token_type: "IDENT", token_literal: "a" }
Token { token_type: ",", token_literal: "," }
Token { token_type: "IDENT", token_literal: "b" }
Token { token_type: ")", token_literal: ")" }
Token { token_type: "{", token_literal: "{" }
Token { token_type: "RETURN", token_literal: "return" }
Token { token_type: "IDENT", token_literal: "a" }
Token { token_type: "+", token_literal: "+" }
Token { token_type: "IDENT", token_literal: "b" }
Token { token_type: ";", token_literal: ";" }
Token { token_type: "}", token_literal: "}" }
>> 
```

Press `Ctrl+D` (EOF) to exit.

## Usage as a Library

```rust
use bwh_rust::lexer::Lexer;
use bwh_rust::token;

let input = "let x = 5;";
let mut lexer = Lexer::new(input.as_bytes().to_vec());

loop {
    let tok = lexer.next_token();
    if tok.token_type == token::EOF {
        break;
    }
    println!("{:?}: {}", tok.token_type, tok.token_literal);
}
```

## Supported Tokens

| Token | Description |
|-------|-------------|
| `ILLEGAL` | Unknown/invalid character |
| `EOF` | End of file/input |
| `IDENT` | Identifiers (variables, functions) |
| `INT` | Integer literals |
| `ASSIGN` | `=` |
| `PLUS` | `+` |
| `MINUS` | `-` |
| `BANG` | `!` |
| `ASTERISK` | `*` |
| `SLASH` | `/` |
| `LT` | `<` |
| `GT` | `>` |
| `EQ` | `==` |
| `NOT_EQ` | `!=` |
| `COMMA` | `,` |
| `SEMICOLON` | `;` |
| `LPAREN` | `(` |
| `RPAREN` | `)` |
| `LBRACE` | `{` |
| `RBRACE` | `}` |
| `FUNCTION` | `fn` keyword |
| `LET` | `let` keyword |
| `IF` | `if` keyword |
| `ELSE` | `else` keyword |
| `TRUE` | `true` keyword |
| `FALSE` | `false` keyword |
| `RETURN` | `return` keyword |

## Implementation Details

### Lexer Algorithm

The lexer uses a single-pass, character-by-character scanning approach:

1. **Input**: Takes a `Vec<u8>` (byte slice of source code)
2. **Position Tracking**: Maintains `position` (current char) and `read_position` (next char)
3. **Token Generation**: `next_token()` returns the next token, skipping whitespace
4. **Identifier/Keyword Recognition**: Reads alphabetic sequences, checks against keyword map
5. **Number Recognition**: Reads consecutive digits
6. **Multi-character Operators**: Handles `==` and `!=` by peeking ahead

### Key Functions

- `Lexer::new(input: Vec<u8>) -> Lexer` - Creates a new lexer
- `lexer.next_token() -> Token` - Returns the next token
- `token::lookup_ident(ident: &str) -> TokenType` - Maps identifiers to keywords

### Token Structure

```rust
pub struct Token {
    pub token_type: TokenType,  // &'static str
    pub token_literal: String,  // The actual text from source
}
```

### Keyword Handling

Keywords are stored in a `LazyLock<HashMap<&str, TokenType>>` for efficient lookup:
- `fn` → `FUNCTION`
- `let` → `LET`
- `if` → `IF`
- `else` → `ELSE`
- `true` → `TRUE`
- `false` → `FALSE`
- `return` → `RETURN`

All other identifiers become `IDENT`.

### AST (Abstract Syntax Tree)

The AST module defines the structure for parsed programs:

- **Program**: Root node containing a vector of statements
- **Statement Trait**: Implemented by `LetStatement`
- **Expression Trait**: Implemented by `Identifier`
- **LetStatement**: Variable declarations with name and value expression
- **Identifier**: Variable/function names

### Parser

The parser uses a recursive descent approach:

- `Parser::new(lexer: Lexer) -> Parser` - Creates a new parser
- `parser.parse_program() -> Option<Program>` - Parses a complete program
- `parser.parse_statement()` - Dispatches to statement-specific parsers
- `parser.parse_let_statement()` - Parses `let` variable declarations
- Error tracking via `errors` field

Currently parses `let` statements. Other statement types (return, expression, etc.) are not yet implemented.

### REPL

The REPL (`repl.rs`) provides an interactive loop:
- Reads lines from stdin
- Tokenizes each line using the lexer
- Prints each token until EOF
- Exits on `Ctrl+D`

## Testing

Tests are in `src/lexer.rs` and `src/parser.rs`:

### Lexer Test (Passing)

Verifies the complete tokenization of a sample program including:
- Variable declarations (`let`)
- Function definitions (`fn`)
- Function calls
- Arithmetic operations
- Comparison operators (`<`, `>`, `==`, `!=`)
- Boolean logic (`!`, `true`, `false`)
- Control flow (`if`, `else`, `return`)

### Parser Test (Failing)

The parser test in `src/parser.rs` tests let statement parsing but uses malformed input:
```rust
let x  5;      // missing '='
let  = 10;     // missing identifier
let  = 838383; // missing identifier
```

This needs to be fixed to test valid let statements like `let x = 5;`.

## Error Handling

- Invalid characters produce `ILLEGAL` tokens
- Parser tracks errors during parsing
- No panic on invalid UTF-8 in identifiers/numbers (uses `unwrap()` - could be improved)
- EOF returns empty `token_literal`

## Future Improvements

- [ ] Fix parser test with valid let statement input
- [ ] Add string literal support
- [ ] Add comment support (`//` and `/* */`)
- [ ] Better error reporting with position info
- [ ] Complete parser implementation (return statements, expressions, function calls, etc.)
- [ ] Interpreter/evaluator
- [ ] Support for more operators and expressions

## License

MIT