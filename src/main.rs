use bwh_rust::{lexer::Lexer, token};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read stdin");

    let mut lexer = Lexer::new(input.into_bytes());

    loop {
        let tok = lexer.next_token();
        if tok.token_type == token::EOF {
            break;
        }
        println!("{:?}\t{}", tok.token_type, tok.token_literal);
    }
}