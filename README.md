# bwh-rust

A simple lexer/tokenizer written in Rust, inspired by the "Writing an Interpreter in Go" book.

## Overview

This project implements a basic lexical analyzer (lexer) that converts source code into tokens. It's a learning project demonstrating fundamental compiler concepts in Rust.

## Features

- **Token Types**: Supports keywords (`LET`, `FUNCTION`), identifiers (`IDENT`), integers (`INT`), operators (`+`, `=`), punctuation (`;`, `,`, `(`, `)`, `{`, `}`), and special tokens (`ILLEGAL`, `EOF`)
- **Lexer**: Character-by-character scanning with position tracking
- **Tests**: Unit tests verifying token recognition

## Project Structure

```
src/
├── lib.rs      # Library entry point, exports lexer and token modules
├── main.rs     # Binary entry point (demo CLI)
├── token.rs    # Token definitions and types
└── lexer.rs    # Lexer implementation with tests
```

## Building

```bash
cargo build
```

## Running Tests

```bash
cargo test
```

## Running the Lexer

The binary reads from stdin and outputs tokens:

```bash
echo 'let x = 5 + 10;' | cargo run
```

Or run interactively:

```bash
cargo run
# Type code, then press Ctrl+D (EOF)
```

### Example Output

```
$ echo 'let x = 5;' | cargo run
"LET"   let
"IDENT" x
"="     =
"INT"   5
";"     ;
```

Each line shows: `TOKEN_TYPE    literal_value`

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
| `COMMA` | `,` |
| `SEMICOLON` | `;` |
| `LPAREN` | `(` |
| `RPAREN` | `)` |
| `LBRACE` | `{` |
| `RBRACE` | `}` |
| `FUNCTION` | `fn` keyword |
| `LET` | `let` keyword |

## Implementation Details

### Lexer Algorithm

The lexer uses a single-pass, character-by-character scanning approach:

1. **Input**: Takes a `Vec<u8>` (byte slice of source code)
2. **Position Tracking**: Maintains `position` (current char) and `read_position` (next char)
3. **Token Generation**: `next_token()` returns the next token, skipping whitespace
4. **Identifier/Keyword Recognition**: Reads alphabetic sequences, checks against keyword map
5. **Number Recognition**: Reads consecutive digits

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

All other identifiers become `IDENT`.

## Testing

Tests are in `src/lexer.rs` and verify the complete tokenization of a sample program:

```rust
let input = "
    let five = 5;
    let ten = 10;
    let add = fn(x, y) {
        x + y;
    }
    let result = add(five, ten);
";
```

Run with `cargo test` - currently 1 test passing.

## Error Handling

- Invalid characters produce `ILLEGAL` tokens
- No panic on invalid UTF-8 in identifiers/numbers (uses `unwrap()` - could be improved)
- EOF returns empty `token_literal`

## Future Improvements

- [ ] Add more operators (`-`, `*`, `/`, `==`, `!=`, `<`, `>`)
- [ ] Add string literal support
- [ ] Add comment support (`//` and `/* */`)
- [ ] Better error reporting with position info
- [ ] Parser implementation (AST generation)

## License

MIT