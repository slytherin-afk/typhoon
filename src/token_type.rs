#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    LeftBraces,
    RightBraces,
    Comma,
    Dot,
    Minus,
    Plus,
    Percentage,
    SemiColon,
    Question,
    Colon,
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
    Var,
    Undefined,
    Function,
    Print,
    Exit,
    NewLine,
    Break,
    Continue,
    Eof,
}
