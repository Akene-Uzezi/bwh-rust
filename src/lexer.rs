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
    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }
    pub fn next_token(&mut self) -> token::Token {
        self.skip_white_space();
        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    token::Token {
                        token_type: token::EQ,
                        token_literal: "==".to_string(),
                    }
                } else {
                    new_token(token::ASSIGN, self.ch)
                }
            }
            b';' => new_token(token::SEMICOLON, self.ch),
            b'(' => new_token(token::LPAREN, self.ch),
            b')' => new_token(token::RPAREN, self.ch),
            b'{' => new_token(token::LBRACE, self.ch),
            b'}' => new_token(token::RBRACE, self.ch),
            b',' => new_token(token::COMMA, self.ch),
            b'+' => new_token(token::PLUS, self.ch),
            b'-' => new_token(token::MINUS, self.ch),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    token::Token {
                        token_type: token::NOT_EQ,
                        token_literal: "!=".to_string(),
                    }
                } else {
                    new_token(token::BANG, self.ch)
                }
            }
            b'/' => new_token(token::SLASH, self.ch),
            b'*' => new_token(token::ASTERISK, self.ch),
            b'<' => new_token(token::LT, self.ch),
            b'>' => new_token(token::GT, self.ch),
            0 => token::Token {
                token_type: token::EOF,
                token_literal: String::new(),
            },
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type: token::TokenType = token::lookup_ident(literal.as_str());
                    return token::Token {
                        token_type,
                        token_literal: literal,
                    };
                }
                if is_digit(self.ch) {
                    let token_type = token::INT;
                    let token_literal = self.read_number();
                    return token::Token {
                        token_type,
                        token_literal,
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
    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        String::from_utf8(self.input[position..self.position].to_vec()).unwrap()
    }
    fn skip_white_space(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
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

fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            }
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            10 == 10;
            10 != 9;
        ";

        let tests = [
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, "="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "ten"),
            (token::ASSIGN, "="),
            (token::INT, "10"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "add"),
            (token::ASSIGN, "="),
            (token::FUNCTION, "fn"),
            (token::LPAREN, "("),
            (token::IDENT, "x"),
            (token::COMMA, ","),
            (token::IDENT, "y"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::IDENT, "x"),
            (token::PLUS, "+"),
            (token::IDENT, "y"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::LET, "let"),
            (token::IDENT, "result"),
            (token::ASSIGN, "="),
            (token::IDENT, "add"),
            (token::LPAREN, "("),
            (token::IDENT, "five"),
            (token::COMMA, ","),
            (token::IDENT, "ten"),
            (token::RPAREN, ")"),
            (token::SEMICOLON, ";"),
            (token::BANG, "!"),
            (token::MINUS, "-"),
            (token::SLASH, "/"),
            (token::ASTERISK, "*"),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::INT, "5"),
            (token::LT, "<"),
            (token::INT, "10"),
            (token::GT, ">"),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::IF, "if"),
            (token::LPAREN, "("),
            (token::INT, "5"),
            (token::LT, "<"),
            (token::INT, "10"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::RETURN, "return"),
            (token::TRUE, "true"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::ELSE, "else"),
            (token::LBRACE, "{"),
            (token::RETURN, "return"),
            (token::FALSE, "false"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::INT, "10"),
            (token::EQ, "=="),
            (token::INT, "10"),
            (token::SEMICOLON, ";"),
            (token::INT, "10"),
            (token::NOT_EQ, "!="),
            (token::INT, "9"),
            (token::SEMICOLON, ";"),
            (token::EOF, ""),
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
