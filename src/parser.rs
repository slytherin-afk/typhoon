use crate::{
    expression::{
        assignment::Assignment, binary::Binary, comma::Comma, grouping::Grouping, literal::Literal,
        logical::Logical, ternary::Ternary, unary::Unary, variable::Variable, Expression,
    },
    object::Object,
    scanner::{
        token::{LiteralType, Token},
        token_type::TokenType,
    },
    stmt::{
        block_stmt::BlockStmt,
        exit_stmt::ExitStmt,
        expression_stmt::ExpressionStmt,
        if_stmt::IfStmt,
        print_stmt::PrintStmt,
        variable_stmt::{VariableDeclaration, VariableStmt},
        while_stmt::WhileStmt,
        Stmt,
    },
    Lib,
};

pub struct Counter {
    current: usize,
    loop_depth: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            current: 0,
            loop_depth: 0,
        }
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

    pub fn parse(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];

        while !self.is_at_end(counter) {
            statements.push(self.declaration_stmt(counter, lib));
        }

        statements.into_iter().collect()
    }

    fn declaration_stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Stmt, ParseError> {
        let stmt = if self.matches(&[TokenType::Var], counter) {
            self.variable_stmt(counter, lib)
        } else {
            self.stmt(counter, lib)
        };

        if let Err(_) = stmt {
            self.synchronize(counter);
        }

        stmt
    }

    fn variable_stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Stmt, ParseError> {
        let mut variables = vec![];
        let mut parse_variable = |counter: &mut Counter, lib: &mut Lib| {
            let name =
                self.consume(&TokenType::Identifier, counter, "Expect an identifier", lib)?;

            let initializer = if self.matches(&[TokenType::Equal], counter) {
                Some(self.assignment(counter, lib)?)
            } else {
                None
            };

            variables.push(VariableDeclaration { name, initializer });

            Ok(())
        };

        parse_variable(counter, lib)?;

        while self.matches(&[TokenType::Comma], counter) {
            parse_variable(counter, lib)?;
        }

        self.consume(&TokenType::SemiColon, counter, "Expect a ';'", lib)?;

        Ok(Stmt::VariableStmt(Box::new(VariableStmt { variables })))
    }

    fn stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::If], counter) {
            self.if_stmt(counter, lib)
        } else if self.matches(&[TokenType::While], counter) {
            self.while_stmt(counter, lib)
        } else if self.matches(&[TokenType::For], counter) {
            self.for_stmt(counter, lib)
        } else if self.matches(&[TokenType::Print], counter) {
            self.print_stmt(counter, lib)
        } else if self.matches(&[TokenType::Exit], counter) {
            self.exit_stmt(counter, lib)
        } else if self.matches(&[TokenType::LeftBraces], counter) {
            Ok(Stmt::BlockStmt(Box::new(BlockStmt {
                stmts: self.block_stmt(counter, lib)?,
            })))
        } else if self.matches(&[TokenType::SemiColon], counter) {
            Ok(Stmt::EmptyStmt)
        } else if self.matches(&[TokenType::Break], counter) {
            if counter.loop_depth == 0 {
                return Err(Self::error(
                    self.previous(counter),
                    "Break can only be used in a loop",
                    lib,
                ));
            }

            self.consume(&TokenType::SemiColon, counter, "Expect a ';'", lib)?;

            Ok(Stmt::BreakStmt)
        } else if self.matches(&[TokenType::Continue], counter) {
            if counter.loop_depth == 0 {
                return Err(Self::error(
                    self.previous(counter),
                    "Continue can only be used in a loop",
                    lib,
                ));
            }

            self.consume(&TokenType::SemiColon, counter, "Expect a ';'", lib)?;

            Ok(Stmt::ContinueStmt)
        } else {
            self.expr_stmt(counter, lib)
        }
    }

    fn if_stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParenthesis, counter, "Expect a '('", lib)?;

        let condition = self.expression(counter, lib)?;

        self.consume(&TokenType::RightParenthesis, counter, "Expect a ')'", lib)?;

        let truth = self.stmt(counter, lib)?;
        let falsy = if self.matches(&[TokenType::Else], counter) {
            Some(self.stmt(counter, lib)?)
        } else {
            None
        };

        Ok(Stmt::IfStmt(Box::new(IfStmt {
            condition,
            truth,
            falsy,
        })))
    }

    fn while_stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Stmt, ParseError> {
        self.consume(
            &TokenType::LeftParenthesis,
            counter,
            "Expect a '(' after while",
            lib,
        )?;

        let condition = self.expression(counter, lib)?;

        self.consume(
            &TokenType::RightParenthesis,
            counter,
            "Expect a ')' before while body",
            lib,
        )?;

        counter.loop_depth += 1;
        let body = self.stmt(counter, lib)?;
        counter.loop_depth -= 1;

        Ok(Stmt::WhileStmt(Box::new(WhileStmt { condition, body })))
    }

    fn for_stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Stmt, ParseError> {
        self.consume(
            &TokenType::LeftParenthesis,
            counter,
            "Expect a '(' after for",
            lib,
        )?;

        let initializer = if self.matches(&[TokenType::SemiColon], counter) {
            None
        } else if self.matches(&[TokenType::Var], counter) {
            Some(self.variable_stmt(counter, lib)?)
        } else {
            Some(self.expr_stmt(counter, lib)?)
        };

        let condition = if self.check(&TokenType::SemiColon, counter) {
            Expression::Literal(Box::new(Literal {
                value: Object::Boolean(true),
            }))
        } else {
            self.expression(counter, lib)?
        };

        self.consume(
            &TokenType::SemiColon,
            counter,
            "Expect a ';' after conditional expression",
            lib,
        )?;

        let increment = if self.check(&TokenType::RightParenthesis, counter) {
            None
        } else {
            Some(self.expression(counter, lib)?)
        };

        self.consume(
            &TokenType::RightParenthesis,
            counter,
            "Expect a ')' before for body",
            lib,
        )?;

        counter.loop_depth += 1;
        let mut body = self.stmt(counter, lib)?;
        counter.loop_depth -= 1;

        if let Some(expression) = increment {
            body = Stmt::BlockStmt(Box::new(BlockStmt {
                stmts: vec![
                    body,
                    Stmt::ExpressionStmt(Box::new(ExpressionStmt { expression })),
                ],
            }));
        }

        body = Stmt::WhileStmt(Box::new(WhileStmt { condition, body }));

        if let Some(initializer) = initializer {
            body = Stmt::BlockStmt(Box::new(BlockStmt {
                stmts: vec![initializer, body],
            }));
        }

        Ok(body)
    }

    fn print_stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Stmt, ParseError> {
        let expression = self.expression(counter, lib)?;

        self.consume(&TokenType::SemiColon, counter, "Expect a ';'", lib)?;

        Ok(Stmt::PrintStmt(Box::new(PrintStmt { expression })))
    }

    fn exit_stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::SemiColon], counter) {
            return Ok(Stmt::ExitStmt(Box::new(ExitStmt { expression: None })));
        }

        let expression = self.expression(counter, lib)?;

        self.consume(&TokenType::SemiColon, counter, "Expect a ';'", lib)?;

        Ok(Stmt::ExitStmt(Box::new(ExitStmt {
            expression: Some(expression),
        })))
    }

    fn block_stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = vec![];

        while !self.check(&TokenType::RightBraces, counter) && !self.is_at_end(counter) {
            stmts.push(self.declaration_stmt(counter, lib)?);
        }

        self.consume(&TokenType::RightBraces, counter, "Expect a '}'", lib)?;

        Ok(stmts)
    }

    fn expr_stmt(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Stmt, ParseError> {
        let expression = self.expression(counter, lib)?;

        self.consume(&TokenType::SemiColon, counter, "Expect a ';'", lib)?;

        Ok(Stmt::ExpressionStmt(Box::new(ExpressionStmt {
            expression,
        })))
    }

    fn expression(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        return self.comma(counter, lib);
    }

    fn comma(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        let mut left = self.assignment(counter, lib)?;

        while self.matches(&[TokenType::Comma], counter) {
            let right = self.assignment(counter, lib)?;
            left = Expression::Comma(Box::new(Comma { left, right }))
        }

        Ok(left)
    }

    fn assignment(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        let variable = self.ternary(counter, lib)?;

        if self.matches(&[TokenType::Equal], counter) {
            match &variable {
                Expression::Variable(variable) => {
                    let expression = self.assignment(counter, lib)?;

                    Ok(Expression::Assignment(Box::new(Assignment {
                        name: variable.name,
                        expression,
                    })))
                }
                _ => Err(Self::error(
                    self.previous(counter),
                    "Invalid left-hand side in assignment",
                    lib,
                )),
            }
        } else {
            Ok(variable)
        }
    }

    pub fn ternary(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        let mut condition = self.or(counter, lib)?;

        if self.matches(&[TokenType::Question], counter) {
            let truth = self.expression(counter, lib)?;

            self.consume(&TokenType::Colon, counter, "Expect a ':'", lib)?;

            let falsy = self.expression(counter, lib)?;

            condition = Expression::Ternary(Box::new(Ternary {
                condition,
                truth,
                falsy,
            }))
        }

        Ok(condition)
    }

    pub fn or(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        let mut left = self.and(counter, lib)?;

        while self.matches(&[TokenType::Or], counter) {
            let operator = self.previous(counter);
            let right = self.and(counter, lib)?;
            left = Expression::Logical(Box::new(Logical {
                operator,
                left,
                right,
            }))
        }

        Ok(left)
    }

    pub fn and(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        let mut left = self.equality(counter, lib)?;

        while self.matches(&[TokenType::And], counter) {
            let operator = self.previous(counter);
            let right = self.equality(counter, lib)?;
            left = Expression::Logical(Box::new(Logical {
                operator,
                left,
                right,
            }))
        }

        Ok(left)
    }

    fn equality(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        let mut left = self.comparison(counter, lib)?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual], counter) {
            let operator = self.previous(counter);
            let right = self.comparison(counter, lib)?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn comparison(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        let mut left = self.term(counter, lib)?;

        while self.matches(
            &[
                TokenType::LessEqual,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::Greater,
            ],
            counter,
        ) {
            let operator = self.previous(counter);
            let right: Expression<'_> = self.term(counter, lib)?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn term(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        let mut left = self.factor(counter, lib)?;

        while self.matches(&[TokenType::Minus, TokenType::Plus], counter) {
            let operator = self.previous(counter);
            let right = self.factor(counter, lib)?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn factor(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        let mut left = self.unary(counter, lib)?;

        while self.matches(&[TokenType::Star, TokenType::Slash], counter) {
            let operator = self.previous(counter);
            let right = self.unary(counter, lib)?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn unary(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus], counter) {
            let operator = self.previous(counter);
            let right = self.unary(counter, lib)?;
            let expression = Expression::Unary(Box::new(Unary { operator, right }));

            Ok(expression)
        } else {
            self.primary(counter, lib)
        }
    }

    fn primary(&self, counter: &mut Counter, lib: &mut Lib) -> Result<Expression, ParseError> {
        if self.matches(&[TokenType::NumberLiteral], counter) {
            let number = self.previous(counter).literal.as_ref().unwrap();

            if let LiteralType::Number(value) = number {
                return Ok(Expression::Literal(Box::new(Literal {
                    value: Object::Number(*value),
                })));
            }
        }

        if self.matches(&[TokenType::StringLiteral], counter) {
            let string = self.previous(counter).literal.as_ref().unwrap();

            if let LiteralType::String(value) = string {
                return Ok(Expression::Literal(Box::new(Literal {
                    value: Object::String(value.to_string()),
                })));
            }
        }

        if self.matches(&[TokenType::False], counter) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: Object::Boolean(false),
            })));
        }

        if self.matches(&[TokenType::True], counter) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: Object::Boolean(true),
            })));
        }

        if self.matches(&[TokenType::Undefined], counter) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: Object::Undefined,
            })));
        }

        if self.matches(&[TokenType::Identifier], counter) {
            return Ok(Expression::Variable(Box::new(Variable {
                name: self.previous(counter),
            })));
        }

        if self.matches(&[TokenType::LeftParenthesis], counter) {
            let expression = self.expression(counter, lib)?;

            self.consume(&TokenType::RightParenthesis, counter, "Expect a ')'", lib)?;

            return Ok(Expression::Grouping(Box::new(Grouping { expression })));
        }

        if self.matches(
            &[
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
                "Binary operator required left hand expression",
                lib,
            );

            return self.expression(counter, lib);
        }

        Err(Self::error(self.peek(counter), "Expect an expression", lib))
    }

    fn matches(&self, tokens: &[TokenType], counter: &mut Counter) -> bool {
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
        token: &TokenType,
        counter: &mut Counter,
        message: &str,
        lib: &mut Lib,
    ) -> Result<&Token, ParseError> {
        if self.check(token, counter) {
            return Ok(self.advance(counter));
        }

        Err(Self::error(self.peek(counter), message, lib))
    }

    fn check(&self, token: &TokenType, counter: &mut Counter) -> bool {
        if self.is_at_end(counter) {
            return false;
        };

        token == &self.peek(counter).token_type
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

    fn error<'b>(token: &'b Token, message: &str, lib: &mut Lib) -> ParseError {
        lib.error_two(token, message);

        ParseError
    }

    fn synchronize(&self, counter: &mut Counter) {
        counter.loop_depth = 0;
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
