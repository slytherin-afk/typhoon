use crate::{
    expression::{
        binary::Binary,
        comma::Comma,
        grouping::Grouping,
        literal::{Literal, LiteralValue},
        ternary::Ternary,
        unary::Unary,
        Expression,
    },
    scanner::{
        token::{LiteralType, Token},
        token_type::TokenType,
    },
    Typhoon,
};

pub struct Counter {
    current: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self { current: 0 }
    }
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
}

#[derive(Debug, Clone)]
pub struct ParseError;

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens }
    }

    pub fn parse(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        return self.expression(counter, typhoon);
    }

    fn expression(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        return self.comma(counter, typhoon);
    }

    fn comma(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut expression = self.ternary(counter, typhoon)?;

        while self.matches(vec![TokenType::Comma], counter) {
            let right = self.ternary(counter, typhoon)?;

            expression = Expression::Comma(Box::new(Comma {
                left: expression,
                right,
            }))
        }

        Ok(expression)
    }

    pub fn ternary(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut expression = self.equality(counter, typhoon)?;

        if self.matches(vec![TokenType::Question], counter) {
            let truth = self.expression(counter, typhoon)?;

            self.consume(
                TokenType::Colon,
                counter,
                "Expected ':' a falsy value",
                typhoon,
            )?;

            let falsy = self.expression(counter, typhoon)?;
            expression = Expression::Ternary(Box::new(Ternary {
                condition: expression,
                truth,
                falsy,
            }))
        }

        Ok(expression)
    }

    fn equality(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut expression = self.comparison(counter, typhoon)?;

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual], counter) {
            let operator = self.previous(counter);
            let right = self.comparison(counter, typhoon)?;
            expression = Expression::Binary(Box::new(Binary {
                left: expression,
                operator,
                right,
            }))
        }

        Ok(expression)
    }

    fn comparison(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut expression = self.term(counter, typhoon)?;

        while self.matches(
            vec![
                TokenType::LessEqual,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::Greater,
            ],
            counter,
        ) {
            let operator = self.previous(counter);
            let right = self.term(counter, typhoon)?;
            expression = Expression::Binary(Box::new(Binary {
                left: expression,
                operator,
                right,
            }))
        }

        Ok(expression)
    }

    fn term(&self, counter: &mut Counter, typhoon: &mut Typhoon) -> Result<Expression, ParseError> {
        let mut expression = self.factor(counter, typhoon)?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus], counter) {
            let operator = self.previous(counter);
            let right = self.factor(counter, typhoon)?;
            expression = Expression::Binary(Box::new(Binary {
                left: expression,
                operator,
                right,
            }))
        }

        Ok(expression)
    }

    fn factor(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut expression = self.unary(counter, typhoon)?;

        while self.matches(vec![TokenType::Star, TokenType::Slash], counter) {
            let operator = self.previous(counter);
            let right = self.unary(counter, typhoon)?;
            expression = Expression::Binary(Box::new(Binary {
                left: expression,
                operator,
                right,
            }))
        }

        Ok(expression)
    }

    fn unary(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus], counter) {
            let operator = self.previous(counter);
            let right = self.unary(counter, typhoon)?;
            let expression = Expression::Unary(Box::new(Unary { operator, right }));

            return Ok(expression);
        }

        self.primary(counter, typhoon)
    }

    fn primary(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        if self.matches(vec![TokenType::NumberLiteral], counter) {
            let number = self.previous(counter).literal.as_ref().unwrap();

            if let LiteralType::Number(value) = number {
                return Ok(Expression::Literal(Box::new(Literal {
                    value: LiteralValue::Number(value),
                })));
            }
        }

        if self.matches(vec![TokenType::StringLiteral], counter) {
            let string = self.previous(counter).literal.as_ref().unwrap();

            if let LiteralType::String(value) = string {
                return Ok(Expression::Literal(Box::new(Literal {
                    value: LiteralValue::String(value),
                })));
            }
        }

        if self.matches(vec![TokenType::False], counter) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: LiteralValue::False,
            })));
        }

        if self.matches(vec![TokenType::True], counter) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: LiteralValue::True,
            })));
        }

        if self.matches(vec![TokenType::None], counter) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: LiteralValue::None,
            })));
        }

        if self.matches(vec![TokenType::LeftParenthesis], counter) {
            let expression = self.expression(counter, typhoon)?;
            self.consume(
                TokenType::RightParenthesis,
                counter,
                "Expect ')' after expression",
                typhoon,
            )?;
            return Ok(Expression::Grouping(Box::new(Grouping { expression })));
        }

        Err(Self::error(
            self.peek(counter),
            "Expect an expression",
            typhoon,
        ))
    }

    fn matches(&self, tokens: Vec<TokenType>, counter: &mut Counter) -> bool {
        for token in tokens {
            if self.check(token, counter) {
                self.advance(counter);

                return true;
            }
        }

        false
    }

    fn consume(
        &self,
        token: TokenType,
        counter: &mut Counter,
        message: &str,
        typhoon: &mut Typhoon,
    ) -> Result<&Token, ParseError> {
        if self.check(token, counter) {
            return Ok(self.advance(counter));
        }

        Err(Self::error(self.peek(counter), message, typhoon))
    }

    fn check(&self, token: TokenType, counter: &mut Counter) -> bool {
        if self.is_at_end(counter) {
            return false;
        };

        token == self.peek(counter).token_type
    }

    fn advance(&self, counter: &mut Counter) -> &Token {
        if !self.is_at_end(counter) {
            counter.current += 1;
        }

        self.previous(counter)
    }

    fn is_at_end(&self, counter: &mut Counter) -> bool {
        counter.current >= self.tokens.len()
    }

    fn peek(&self, counter: &mut Counter) -> &Token {
        &self.tokens[counter.current]
    }

    fn previous(&self, counter: &mut Counter) -> &Token {
        &self.tokens[counter.current - 1]
    }

    fn error(token: &Token, message: &str, typhoon: &mut Typhoon) -> ParseError {
        typhoon.error_two(token, message);

        ParseError
    }

    fn _synchronize(&self, counter: &mut Counter) {
        self.advance(counter);

        while !self.is_at_end(counter) {
            match self.peek(counter).token_type {
                TokenType::Class
                | TokenType::If
                | TokenType::While
                | TokenType::For
                | TokenType::Let
                | TokenType::Function
                | TokenType::Return => return,
                TokenType::SemiColon => {
                    self.advance(counter);

                    return;
                }
                _ => self.advance(counter),
            };
        }
    }
}
