use std::fmt;

#[derive(Clone, Debug)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    LeftBraces,
    RightBraces,
    Coma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    StringLiteral,
    NumberLiteral,
    And,
    Or,
    Class,
    If,
    Else,
    True,
    False,
    While,
    For,
    Return,
    Super,
    This,
    Let,
    None,
    Function,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
