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

#[cfg(test)]
mod parsertest {
    use super::*;

    #[test]
    fn test_let_statement() {
        let input = String::from(
            "
    	    let x = 5;
          let y = 10;
    	    let foobar = 838383;
        ",
        );
        let input_as_bytes = input.as_bytes();
        let l = lexer::Lexer::new(input_as_bytes.to_vec());
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("parse_program returned None");
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements got={}",
            program.statements.len()
        );
        let tests = [("x"), ("y"), ("foobar")];
        for (i, expected_identifier) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            assert!(test_statement(stmt, expected_identifier.to_string()));
        }
    }

    fn test_statement(s: &Box<dyn ast::Statement>, name: String) -> bool {
        if s.token_literal() != "let" {
            println!("s.token_literal not 'let' got={}", s.token_literal());
            return false;
        }

        let let_stmt: ast::LetStatement;
        return false;
    }
}
