pub mod token;
pub mod token_type;

use crate::Typhoon;
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
    "let" => TokenType::Let,
    "none" => TokenType::None,
    "fn" => TokenType::Function,
};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    current: usize,
    start: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self, typhoon: &mut Typhoon) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token(typhoon);
        }

        self.add_token(TokenType::Eof);
        self.tokens
    }

    fn scan_token(&mut self, typhoon: &mut Typhoon) {
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
            self.add_token(TokenType::Coma);
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
            self.slash(typhoon)
        } else if c == '\n' {
            self.line += 1;
        } else if c == '"' {
            self.string_literal(typhoon);
        } else if c.is_digit(10) {
            self.number_literal();
        } else if Self::is_alphabetic(c) {
            self.identifier();
        } else if c == ' ' || c == '\r' || c == '\t' {
        } else {
            typhoon.error_one(self.line, "Unexpected Character Error");
        }
    }

    fn slash(&mut self, typhoon: &mut Typhoon) {
        if self.peek() == '/' {
            loop {
                self.advance();

                if self.peek() == '\n' || self.is_at_end() {
                    break;
                }
            }
        } else if self.peek() == '*' {
            loop {
                self.advance();

                if self.is_at_end() {
                    typhoon.error_one(self.line, "Unexpected Character Error");

                    break;
                }

                if self.peek() == '*' && self.peek_next() == '/' {
                    self.advance();
                    self.advance();

                    break;
                }
            }
        } else {
            self.add_token(TokenType::Slash);
        }
    }

    fn string_literal(&mut self, typhoon: &mut Typhoon) {
        loop {
            if self.peek() == '"' {
                self.advance();

                break;
            }

            if self.is_at_end() {
                typhoon.error_one(self.line, "Unexpected Character Error");

                break;
            }

            self.advance();
        }

        let literal = &self.source[self.start + 1..self.current - 1];

        self.add_token_with_literal(TokenType::StringLiteral, Some(LiteralType::String(literal)));
    }

    fn number_literal(&mut self) {
        loop {
            if self.peek().is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            loop {
                if self.peek().is_digit(10) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let number = self.source[self.start..self.current]
            .parse()
            .expect("a number literal");

        self.add_token_with_literal(TokenType::NumberLiteral, Some(LiteralType::Number(number)));
    }

    fn identifier(&mut self) {
        loop {
            let peek = self.peek();

            if Self::is_alphabetic(peek) || peek.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.source[self.start..self.current];
        let token_type = if let Some(token_type) = KEYWORDS.get(lexeme) {
            *token_type
        } else {
            TokenType::Identifier
        };

        self.add_token(token_type);
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() == expected {
            self.current += 1;

            return true;
        }

        false
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
        let c = self
            .source
            .chars()
            .nth(self.current)
            .expect("source have a character");

        self.current += 1;

        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<LiteralType<'a>>) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }
}
