//token
use std::collections::HashMap;
use std::sync::LazyLock;
static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> =
    LazyLock::new(|| HashMap::from([("fn", FUNCTION), ("let", LET)]));
pub type TokenType = &'static str;
pub struct Token {
    pub token_type: TokenType,
    pub token_literal: String,
}

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";

//Operators
pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const MINUS: TokenType = "-";
pub const BANG: TokenType = "!";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";
pub const LT: TokenType = "<";
pub const GT: TokenType = ">";

pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";

pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";

//Keywords
pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";

pub fn lookup_ident(ident: &str) -> TokenType {
    if let Some(&tok) = KEYWORDS.get(ident) {
        return tok;
    }
    IDENT
}
