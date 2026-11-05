use crate::ast;
use crate::lexer;
use crate::token;

pub struct Parser {
    l: lexer::Lexer,

    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(mut l: lexer::Lexer) -> Self {
        let cur_token = l.next_token();
        let peek_token = l.next_token();

        Parser {
            l,
            cur_token,
            peek_token,
        }
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        None
    }
}
