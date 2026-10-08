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
├── main.rs     # Binary entry point (currently prints "Hello, world!")
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

## Usage

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

## License

MIT