use crate::token;
pub struct Lexer {
    pub input: Vec<u8>,
    pub position: usize,
    pub read_position: usize,
    pub ch: u8,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn next_token(&mut self) -> token::Token {
        let tok = match self.ch {
            b'=' => new_token(token::ASSIGN, self.ch),
            b';' => new_token(token::SEMICOLON, self.ch),
            b'(' => new_token(token::LPAREN, self.ch),
            b')' => new_token(token::RPAREN, self.ch),
            b'{' => new_token(token::LPAREN, self.ch),
            b',' => new_token(token::COMMA, self.ch),
            b'+' => new_token(token::PLUS, self.ch),
            0 => token::Token {
                token_type: token::EOF,
                token_literal: String::new(),
            },
            _ => new_token(token::ILLEGAL, self.ch),
        };
        self.read_char();
        tok
    }
}

fn new_token(token_type: token::TokenType, ch: u8) -> token::Token {
    token::Token {
        token_type,
        token_literal: (ch as char).to_string(),
    }
}
