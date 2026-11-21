use crate::ast;
use crate::lexer;
use crate::token;

pub struct Parser {
    l: lexer::Lexer,

    errors: Option<Vec<String>>,

    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(mut l: lexer::Lexer) -> Self {
        let cur_token = l.next_token();
        let peek_token = l.next_token();

        Parser {
            l,
            errors: None,
            cur_token,
            peek_token,
        }
    }

    pub fn errors(self) -> Vec<String> {
        self.errors.expect("parser::errors returned None")
    }

    fn peek_error(&mut self, t: token::TokenType) {
        let message = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        );
        self.errors.get_or_insert_with(Vec::new).push(message);
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program {
            statements: Vec::new(),
        };

        while self.cur_token.token_type != token::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.token_type {
            token::LET => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(token::IDENT) {
            return None;
        }

        let name = ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.token_literal.clone(),
        };

        if !self.expect_peek(token::ASSIGN) {
            return None;
        }

        while self.cur_token.token_type != token::SEMICOLON
            && self.cur_token.token_type != token::EOF
        {
            self.next_token();
        }

        Some(Box::new(ast::LetStatement {
            token,
            name: Some(name),
            value: None,
        }))
    }

    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token.token_type == t {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }
}

#[cfg(test)]
mod parsertest {
    use crate::ast::Node;

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
        check_parser_errors(&p);
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

    fn check_parser_errors(p: &Parser) {
        let Some(errors) = &p.errors else {
            return;
        };

        if errors.is_empty() {
            return;
        }
        eprintln!("Parser has {} errors", errors.iter().len());
        for msg in errors {
            eprintln!("parser error: {}", msg);
        }
        panic!("Parser has encountered {} error(s)", errors.len());
    }

    fn test_statement(s: &Box<dyn ast::Statement>, name: String) -> bool {
        if s.token_literal() != "let" {
            println!("s.token_literal not 'let' got={}", s.token_literal());
            return false;
        }

        let Some(let_stmt) = s.as_any().downcast_ref::<ast::LetStatement>() else {
            println!("s not ast::LetStatement");
            return false;
        };

        let Some(ident) = &let_stmt.name else {
            println!("let_stmt.name is None");
            return false;
        };

        if ident.value != name {
            println!("s.name not '{}'. got={}", name, ident.token_literal());
            return false;
        }

        if ident.token_literal() != name {
            println!("s.name not '{}'. got={}", name, ident.token_literal());
            return false;
        }
        true
    }

    #[test]
    fn test_return_statement() {
        let input = String::from(
            "
            return 5;
            return 10;
            return 9933322;
        ",
        );

        let inputbytes = &input.as_bytes();
        let l = lexer::Lexer::new(inputbytes.to_vec());
        let mut p = Parser::new(l);
        let programoption = p.parse_program();
        check_parser_errors(&p);

        let Some(program) = &programoption else {
            return;
        };

        if program.statements.iter().len() != 3 {
            eprintln!(
                "program.statements does not contain 3 statments got {}",
                program.statements.iter().len()
            );
            panic!("program.statements does not contain 3 statements");
        }

        for statment in program.statements.iter().enumerate() {
            let Some(returnStmt) = statment.1.as_any().downcast_ref::<ast::ReturnStatement>()
            else {
                println!("stmt not ast::ReturnStatement");
                continue;
            };
            if returnStmt.token_literal() != "return" {
                eprintln!(
                    "returnStmt.token_literal not 'return', got {}",
                    returnStmt.token_literal()
                );
                panic!("returnStmt.token_literal not 'return'")
            }
        }
    }
}
