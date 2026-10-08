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
            b'{' => new_token(token::LBRACE, self.ch),
            b'}' => new_token(token::RBRACE, self.ch),
            b',' => new_token(token::COMMA, self.ch),
            b'+' => new_token(token::PLUS, self.ch),
            0 => token::Token {
                token_type: token::EOF,
                token_literal: String::new(),
            },
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return token::Token {
                        token_type: token::IDENT,
                        token_literal: literal,
                    };
                } else {
                    new_token(token::ILLEGAL, self.ch)
                }
            }
        };
        self.read_char();
        tok
    }
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_utf8(self.input[position..self.position].to_vec()).unwrap()
    }
}

fn new_token(token_type: token::TokenType, ch: u8) -> token::Token {
    token::Token {
        token_type,
        token_literal: (ch as char).to_string(),
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){};";

        let tests = [
            (token::ASSIGN, "="),
            (token::PLUS, "+"),
            (token::LPAREN, "("),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::RBRACE, "}"),
            (token::SEMICOLON, ";"),
        ];

        let mut l = Lexer::new(input.as_bytes().to_vec());
        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = l.next_token();

            assert_eq!(
                tok.token_type, *expected_type,
                "tests[{}] tokentype wrong",
                i
            );

            assert_eq!(
                tok.token_literal,
                *expected_literal.to_string(),
                "test[{}] tokenliteral wrong",
                i
            );
        }
    }
}
