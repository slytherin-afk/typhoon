use serde::Serialize;

use super::token_type::TokenType;

#[derive(Serialize)]
#[serde(untagged)]
pub enum LiteralType<'a> {
    String(&'a str),
    Number(f64),
}

#[derive(Serialize)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: Option<LiteralType<'a>>,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Option<LiteralType<'a>>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
