use crate::{
    expression::{
        assignment::Assignment, binary::Binary, comma::Comma, grouping::Grouping, literal::Literal,
        ternary::Ternary, unary::Unary, variable::Variable, Expression,
    },
    object::Object,
    scanner::{
        token::{LiteralType, Token},
        token_type::TokenType,
    },
    stmt::{
        block_stmt::BlockStmt, exit_stmt::ExitStmt, expression_stmt::ExpressionStmt,
        print_stmt::PrintStmt, variable_stmt::VariableStmt, Stmt,
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

pub struct Parser {
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct ParseError;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];

        while !self.is_at_end(counter) {
            statements.push(self.declaration_stmt(counter, typhoon)?);
        }

        Ok(statements)
    }

    fn expression(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        return self.assignment(counter, typhoon);
    }

    fn declaration_stmt(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Stmt, ParseError> {
        let stmt = if self.matches(vec![TokenType::Var], counter) {
            self.variable_stmt(counter, typhoon)
        } else {
            self.stmt(counter, typhoon)
        };

        if let Err(_) = stmt {
            self.synchronize(counter);
        }

        stmt
    }

    fn stmt(&self, counter: &mut Counter, typhoon: &mut Typhoon) -> Result<Stmt, ParseError> {
        if self.matches(vec![TokenType::Print], counter) {
            self.print_stmt(counter, typhoon)
        } else if self.matches(vec![TokenType::Exit], counter) {
            self.exit_stmt(counter, typhoon)
        } else if self.matches(vec![TokenType::LeftBraces], counter) {
            Ok(Stmt::BlockStmt(Box::new(BlockStmt {
                stmts: self.block_stmt(counter, typhoon)?,
            })))
        } else {
            self.expr_stmt(counter, typhoon)
        }
    }

    fn print_stmt(&self, counter: &mut Counter, typhoon: &mut Typhoon) -> Result<Stmt, ParseError> {
        let expression = self.expression(counter, typhoon)?;

        self.consume(
            TokenType::SemiColon,
            counter,
            "Expected ';' at the end of print expression",
            typhoon,
        )?;

        Ok(Stmt::PrintStmt(Box::new(PrintStmt { expression })))
    }

    fn exit_stmt(&self, counter: &mut Counter, typhoon: &mut Typhoon) -> Result<Stmt, ParseError> {
        if self.matches(vec![TokenType::SemiColon], counter) {
            return Ok(Stmt::ExitStmt(Box::new(ExitStmt { expression: None })));
        }

        let expression = self.expression(counter, typhoon)?;

        self.consume(
            TokenType::SemiColon,
            counter,
            "Expected ';' at the end of exit expression",
            typhoon,
        )?;

        Ok(Stmt::ExitStmt(Box::new(ExitStmt {
            expression: Some(expression),
        })))
    }

    fn variable_stmt(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Stmt, ParseError> {
        let mut names = vec![self.consume(
            TokenType::Identifier,
            counter,
            "Expected an identifier",
            typhoon,
        )?];

        while self.matches(vec![TokenType::Comma], counter) {
            let identifier = self.consume(
                TokenType::Identifier,
                counter,
                "Expected an identifier",
                typhoon,
            )?;

            names.push(identifier);
        }

        let initializer = if self.matches(vec![TokenType::Equal], counter) {
            Some(self.expression(counter, typhoon)?)
        } else {
            None
        };

        self.consume(
            TokenType::SemiColon,
            counter,
            "Expected ';' at the end of variable declaration",
            typhoon,
        )?;

        Ok(Stmt::VariableStmt(Box::new(VariableStmt {
            names,
            initializer,
        })))
    }

    fn expr_stmt(&self, counter: &mut Counter, typhoon: &mut Typhoon) -> Result<Stmt, ParseError> {
        let expression = self.expression(counter, typhoon)?;

        self.consume(
            TokenType::SemiColon,
            counter,
            "Expect ';' at the end of expression statement",
            typhoon,
        )?;

        Ok(Stmt::ExpressionStmt(Box::new(ExpressionStmt {
            expression,
        })))
    }

    fn block_stmt(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = vec![];

        while !self.check(TokenType::RightBraces, counter) && !self.is_at_end(counter) {
            stmts.push(self.declaration_stmt(counter, typhoon)?);
        }

        self.consume(
            TokenType::RightBraces,
            counter,
            "Expect '}' after block",
            typhoon,
        )?;

        Ok(stmts)
    }

    fn assignment(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let left = self.comma(counter, typhoon)?;

        if self.matches(vec![TokenType::Equal], counter) {
            match &left {
                Expression::Variable(variable) => {
                    let right = self.assignment(counter, typhoon)?;

                    Ok(Expression::Assignment(Box::new(Assignment {
                        name: variable.name,
                        expression: right,
                    })))
                }
                _ => Err(Self::error(
                    self.previous(counter),
                    "Invalid assignment target",
                    typhoon,
                )),
            }
        } else {
            Ok(left)
        }
    }

    fn comma(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut left = self.ternary(counter, typhoon)?;

        while self.matches(vec![TokenType::Comma], counter) {
            let right = self.ternary(counter, typhoon)?;

            left = Expression::Comma(Box::new(Comma { left, right }))
        }

        Ok(left)
    }

    pub fn ternary(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut condition = self.equality(counter, typhoon)?;

        if self.matches(vec![TokenType::Question], counter) {
            let truth = self.expression(counter, typhoon)?;

            self.consume(
                TokenType::Colon,
                counter,
                "Expected ':' a falsy value",
                typhoon,
            )?;

            let falsy = self.expression(counter, typhoon)?;

            condition = Expression::Ternary(Box::new(Ternary {
                condition,
                truth,
                falsy,
            }))
        }

        Ok(condition)
    }

    fn equality(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut left = self.comparison(counter, typhoon)?;

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual], counter) {
            let operator = self.previous(counter);
            let right = self.comparison(counter, typhoon)?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn comparison(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut left = self.term(counter, typhoon)?;

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
            let right: Expression<'_> = self.term(counter, typhoon)?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn term(&self, counter: &mut Counter, typhoon: &mut Typhoon) -> Result<Expression, ParseError> {
        let mut left = self.factor(counter, typhoon)?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus], counter) {
            let operator = self.previous(counter);
            let right = self.factor(counter, typhoon)?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn factor(
        &self,
        counter: &mut Counter,
        typhoon: &mut Typhoon,
    ) -> Result<Expression, ParseError> {
        let mut left = self.unary(counter, typhoon)?;

        while self.matches(vec![TokenType::Star, TokenType::Slash], counter) {
            let operator = self.previous(counter);
            let right = self.unary(counter, typhoon)?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
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

            Ok(expression)
        } else {
            self.primary(counter, typhoon)
        }
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
                    value: Object::Number(*value),
                })));
            }
        }

        if self.matches(vec![TokenType::StringLiteral], counter) {
            let string = self.previous(counter).literal.as_ref().unwrap();

            if let LiteralType::String(value) = string {
                return Ok(Expression::Literal(Box::new(Literal {
                    value: Object::String(value.to_string()),
                })));
            }
        }

        if self.matches(vec![TokenType::False], counter) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: Object::Boolean(false),
            })));
        }

        if self.matches(vec![TokenType::True], counter) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: Object::Boolean(true),
            })));
        }

        if self.matches(vec![TokenType::Undefined], counter) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: Object::Undefined,
            })));
        }

        if self.matches(vec![TokenType::Identifier], counter) {
            return Ok(Expression::Variable(Box::new(Variable {
                name: self.previous(counter),
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

        if self.matches(
            vec![
                TokenType::EqualEqual,
                TokenType::BangEqual,
                TokenType::LessEqual,
                TokenType::Less,
                TokenType::GreaterEqual,
                TokenType::Greater,
                TokenType::Plus,
                TokenType::Star,
                TokenType::Slash,
            ],
            counter,
        ) {
            Self::error(
                self.peek(counter),
                "Unexpected without left operand",
                typhoon,
            );

            return self.expression(counter, typhoon);
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
        self.peek(counter).token_type == TokenType::Eof
    }

    fn peek(&self, counter: &mut Counter) -> &Token {
        &self.tokens[counter.current]
    }

    fn previous(&self, counter: &mut Counter) -> &Token {
        &self.tokens[counter.current - 1]
    }

    fn error<'b>(token: &'b Token, message: &str, typhoon: &mut Typhoon) -> ParseError {
        typhoon.error_two(token, message);

        ParseError
    }

    fn synchronize(&self, counter: &mut Counter) {
        self.advance(counter);

        while !self.is_at_end(counter) {
            match self.peek(counter).token_type {
                TokenType::Class
                | TokenType::If
                | TokenType::While
                | TokenType::For
                | TokenType::Var
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
