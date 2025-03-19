use super::token_type::TokenType;

#[derive(Clone)]
pub enum LiteralType {
    String(String),
    Number(f64),
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
    pub line: usize,
    pub identifier_hash: Option<String>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralType>,
        line: usize,
        identifier_hash: Option<String>,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            identifier_hash,
        }
    }
}
