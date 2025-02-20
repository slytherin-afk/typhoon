#![allow(dead_code)]

use std::fmt;

use super::token_type::TokenType;

pub enum LiteralType {
    String(String),
    Number(f64),
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralType>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralType>,
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.token_type)?;

        if let Some(literal) = &self.literal {
            match literal {
                LiteralType::String(s) => write!(f, "{s}",)?,
                LiteralType::Number(u) => write!(f, "{u}",)?,
            };
        } else {
            write!(f, "{}", self.lexeme)?
        }

        write!(f, "</{}>", self.token_type)
    }
}
