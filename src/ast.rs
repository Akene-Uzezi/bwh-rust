use crate::token;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::Write;
pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }
    fn string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            write!(out, "{}", s.string()).unwrap();
        }
        out
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>,
}

pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Option<Box<dyn Expression>>,
}

pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.token_literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push(' ');
        if let Some(name) = &self.name {
            out.push_str(&name.string());
        };
        out.push_str(" = ");
        if let Some(value) = &self.value {
            out.push_str(&value.string());
        };
        out.push(';');
        out
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.token_literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push(' ');
        if let Some(value) = &self.return_value {
            out.push_str(&value.string());
        };
        out.push(';');
        out
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.token_literal.clone()
    }

    fn string(&self) -> String {
        if let Some(expr) = &self.expression {
            expr.string()
        } else {
            String::new()
        }
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.token_literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

#[cfg(test)]
mod asttest {
    use super::*;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: token::Token {
                    token_type: token::LET,
                    token_literal: "let".to_string(),
                },
                name: Some(Identifier {
                    token: token::Token {
                        token_type: token::IDENT,
                        token_literal: "my_var".to_string(),
                    },
                    value: "myVar".to_string(),
                }),
                value: Some(Box::new(Identifier {
                    token: token::Token {
                        token_type: token::IDENT,
                        token_literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                })),
            })],
        };
        let expected = "let myVar = anotherVar;";
        assert_eq!(program.string(), expected)
    }
}
