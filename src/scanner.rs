pub mod token;
pub mod token_type;

use crate::Lib;
use phf::phf_map;
use token::{LiteralType, Token};
use token_type::TokenType;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "or" => TokenType::Or,
    "class" => TokenType::Class,
    "if" => TokenType::If,
    "else" => TokenType::Else,
    "true" => TokenType::True,
    "false" => TokenType::False,
    "while" => TokenType::While,
    "for" => TokenType::For,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "var" => TokenType::Var,
    "undefined" => TokenType::Undefined,
    "function" => TokenType::Function,
    "print" => TokenType::Print,
    "exit" => TokenType::Exit,
    "break" => TokenType::Break,
    "continue" => TokenType::Continue,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token();
        }

        self.add_token(TokenType::Eof);
        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        if c == '(' {
            self.add_token(TokenType::LeftParenthesis);
        } else if c == ')' {
            self.add_token(TokenType::RightParenthesis);
        } else if c == '{' {
            self.add_token(TokenType::LeftBraces);
        } else if c == '}' {
            self.add_token(TokenType::RightBraces);
        } else if c == ',' {
            self.add_token(TokenType::Comma);
        } else if c == '.' {
            self.add_token(TokenType::Dot);
        } else if c == '-' {
            self.add_token(TokenType::Minus);
        } else if c == '+' {
            self.add_token(TokenType::Plus);
        } else if c == '*' {
            self.add_token(TokenType::Star);
        } else if c == ';' {
            self.add_token(TokenType::SemiColon);
        } else if c == '?' {
            self.add_token(TokenType::Question);
        } else if c == ':' {
            self.add_token(TokenType::Colon);
        } else if c == '!' {
            let token_type = if self.matches('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            };
            self.add_token(token_type);
        } else if c == '=' {
            let token_type = if self.matches('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            };
            self.add_token(token_type);
        } else if c == '<' {
            let token_type = if self.matches('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            };
            self.add_token(token_type);
        } else if c == '>' {
            let token_type = if self.matches('=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            };
            self.add_token(token_type);
        } else if c == '/' {
            self.slash()
        } else if c == '\n' {
            self.line += 1;
        } else if c == '"' {
            self.string_literal();
        } else if c.is_digit(10) {
            self.number_literal();
        } else if Self::is_alphabetic(c) {
            self.identifier();
        } else if c == ' ' || c == '\r' || c == '\t' {
        } else {
            Lib::error_one(self.line, "Unexpected character");
        }
    }

    fn slash(&mut self) {
        match self.peek() {
            '/' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            '*' => {
                self.advance();

                while !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }

                    if self.peek() == '*' && self.peek_next() == '/' {
                        self.advance();
                        self.advance();

                        return;
                    }

                    self.advance();
                }

                Lib::error_one(self.line, "Expect a '*/'");
            }
            _ => {
                self.add_token(TokenType::Slash);
            }
        }
    }

    fn string_literal(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                '"' => {
                    self.advance();

                    let literal = &self.source[self.start + 1..self.current - 1];

                    self.add_token_with_literal(
                        TokenType::StringLiteral,
                        Some(LiteralType::String(literal.to_string())),
                    );

                    return;
                }
                '\n' => {
                    break;
                }
                _ => {
                    self.advance();
                }
            }
        }

        Lib::error_one(self.line, "Unterminated string literal");
    }

    fn number_literal(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let number = self.source[self.start..self.current]
            .parse()
            .expect("Valid number literal");

        self.add_token_with_literal(TokenType::NumberLiteral, Some(LiteralType::Number(number)));
    }

    fn identifier(&mut self) {
        while Self::is_alphabetic(self.peek()) || self.peek().is_digit(10) {
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current];
        let token_type = if let Some(token_type) = KEYWORDS.get(lexeme) {
            token_type.clone()
        } else {
            TokenType::Identifier
        };

        self.add_token(token_type);
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.current += 1;

            true
        }
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn is_alphabetic(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();

        self.current += 1;

        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<LiteralType>) {
        let lexeme = self.source[self.start..self.current].to_string();
        let token = Token::new(token_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }
}
