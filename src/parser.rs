use std::cell::RefCell;

use crate::{
    expression::{
        assignment::Assignment, binary::Binary, call::Call, comma::Comma, grouping::Grouping,
        literal::Literal, logical::Logical, ternary::Ternary, unary::Unary, variable::Variable,
        Expression,
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
        function_stmt::FunctionStmt,
        if_stmt::IfStmt,
        print_stmt::PrintStmt,
        variable_stmt::{VariableDeclaration, VariableStmt},
        while_stmt::WhileStmt,
        Stmt,
    },
    Lib,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: RefCell<usize>,
    loop_depth: RefCell<usize>,
    in_function: RefCell<bool>,
}

#[derive(Debug)]
pub struct ParseError;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: RefCell::new(0),
            loop_depth: RefCell::new(0),
            in_function: RefCell::new(false),
        }
    }

    pub fn parse(&self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];

        while !self.is_at_end() {
            statements.push(self.declaration_stmt());
        }

        statements.into_iter().collect()
    }

    fn declaration_stmt(&self) -> Result<Stmt, ParseError> {
        let stmt = if self.matches(&[TokenType::Var]) {
            self.variable_stmt()
        } else {
            self.stmt()
        };

        if let Err(_) = stmt {
            self.synchronize();
        }

        stmt
    }

    fn variable_stmt(&self) -> Result<Stmt, ParseError> {
        let mut variables = vec![];
        let mut parse_variable = || {
            let name = self
                .consume(&TokenType::Identifier, "Expect an identifier")?
                .clone();
            let initializer = if self.matches(&[TokenType::Equal]) {
                Some(self.assignment()?)
            } else {
                None
            };

            variables.push(VariableDeclaration { name, initializer });

            Ok(())
        };

        parse_variable()?;

        while self.matches(&[TokenType::Comma]) {
            parse_variable()?;
        }

        self.consume(&TokenType::SemiColon, "Expect a ';'")?;

        Ok(Stmt::VariableStmt(Box::new(VariableStmt { variables })))
    }

    fn stmt(&self) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::If]) {
            self.if_stmt()
        } else if self.matches(&[TokenType::Function]) {
            self.function_stmt("function")
        } else if self.matches(&[TokenType::While]) {
            self.while_stmt()
        } else if self.matches(&[TokenType::For]) {
            self.for_stmt()
        } else if self.matches(&[TokenType::Print]) {
            self.print_stmt()
        } else if self.matches(&[TokenType::Exit]) {
            self.exit_stmt()
        } else if self.matches(&[TokenType::LeftBraces]) {
            Ok(Stmt::BlockStmt(Box::new(BlockStmt {
                stmts: self.block_stmt()?,
            })))
        } else if self.matches(&[TokenType::SemiColon]) {
            Ok(Stmt::EmptyStmt)
        } else if self.matches(&[TokenType::Break]) {
            if *self.loop_depth.borrow() == 0 || *self.in_function.borrow() {
                Self::error(self.previous(), "Break can only be used in a loop");
            }

            self.consume(&TokenType::SemiColon, "Expect a ';' at the end of break")?;

            Ok(Stmt::BreakStmt)
        } else if self.matches(&[TokenType::Continue]) {
            if *self.loop_depth.borrow() == 0 || *self.in_function.borrow() {
                Self::error(self.previous(), "Continue can only be used in a loop");
            }

            self.consume(&TokenType::SemiColon, "Expect a ';' at the of continue")?;

            Ok(Stmt::ContinueStmt)
        } else {
            self.expr_stmt()
        }
    }

    fn block_stmt(&self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = vec![];

        while !self.check(&TokenType::RightBraces) && !self.is_at_end() {
            stmts.push(self.declaration_stmt()?);
        }

        self.consume(&TokenType::RightBraces, "Expect a '}' at the end of block")?;

        Ok(stmts)
    }

    fn if_stmt(&self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParenthesis, "Expect a '(' after if")?;

        let condition = self.expression()?;

        self.consume(&TokenType::RightParenthesis, "Expect a ')' before if body")?;

        let truth = self.stmt()?;
        let falsy = if self.matches(&[TokenType::Else]) {
            Some(self.stmt()?)
        } else {
            None
        };

        Ok(Stmt::IfStmt(Box::new(IfStmt {
            condition,
            truth,
            falsy,
        })))
    }

    fn function_stmt(&self, kind: &str) -> Result<Stmt, ParseError> {
        *self.in_function.borrow_mut() = true;

        let name = self
            .consume(&TokenType::Identifier, &format!("Expect {kind} name"))?
            .clone();

        self.consume(
            &TokenType::LeftParenthesis,
            &format!("Expect '(' after {kind} name"),
        )?;

        let mut params = vec![];

        if !self.check(&TokenType::RightParenthesis) {
            loop {
                if params.len() >= 255 {
                    Self::error(self.peek(), "Can't have more than 255 parameters");
                }

                let param = self
                    .consume(
                        &TokenType::Identifier,
                        &format!("Expect identifier after {kind} name"),
                    )?
                    .clone();

                params.push(param);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(
            &TokenType::RightParenthesis,
            &format!("Expect ')' after {kind} params"),
        )?;

        self.consume(
            &TokenType::LeftBraces,
            &format!("Expect '{{' after {kind} params"),
        )?;

        let body = self.block_stmt()?;

        Ok(Stmt::FunctionStmt(Box::new(FunctionStmt {
            name,
            params,
            body,
        })))
    }

    fn while_stmt(&self) -> Result<Stmt, ParseError> {
        *self.in_function.borrow_mut() = false;

        self.consume(&TokenType::LeftParenthesis, "Expect a '(' after while")?;

        let condition = self.expression()?;

        self.consume(
            &TokenType::RightParenthesis,
            "Expect a ')' before while body",
        )?;

        *self.loop_depth.borrow_mut() += 1;
        let body = self.stmt()?;
        *self.loop_depth.borrow_mut() -= 1;

        Ok(Stmt::WhileStmt(Box::new(WhileStmt { condition, body })))
    }

    fn for_stmt(&self) -> Result<Stmt, ParseError> {
        *self.in_function.borrow_mut() = false;

        self.consume(&TokenType::LeftParenthesis, "Expect a '(' after for")?;

        let initializer = if self.matches(&[TokenType::SemiColon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.variable_stmt()?)
        } else {
            Some(self.expr_stmt()?)
        };

        let condition = if self.check(&TokenType::SemiColon) {
            Expression::Literal(Box::new(Literal {
                value: Object::Boolean(true),
            }))
        } else {
            self.expression()?
        };

        self.consume(
            &TokenType::SemiColon,
            "Expect a ';' after conditional expression",
        )?;

        let increment = if self.check(&TokenType::RightParenthesis) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&TokenType::RightParenthesis, "Expect a ')' before for body")?;

        *self.loop_depth.borrow_mut() += 1;
        let mut body = self.stmt()?;
        *self.loop_depth.borrow_mut() -= 1;

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

    fn print_stmt(&self) -> Result<Stmt, ParseError> {
        let expression = self.expression()?;

        self.consume(&TokenType::SemiColon, "Expect a ';' at the end of print")?;

        Ok(Stmt::PrintStmt(Box::new(PrintStmt { expression })))
    }

    fn exit_stmt(&self) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::SemiColon]) {
            return Ok(Stmt::ExitStmt(Box::new(ExitStmt { expression: None })));
        }

        let expression = self.expression()?;

        self.consume(&TokenType::SemiColon, "Expect a ';' at the end of exit")?;

        Ok(Stmt::ExitStmt(Box::new(ExitStmt {
            expression: Some(expression),
        })))
    }

    fn expr_stmt(&self) -> Result<Stmt, ParseError> {
        let expression = self.expression()?;

        self.consume(
            &TokenType::SemiColon,
            "Expect a ';' at the end of expression",
        )?;

        Ok(Stmt::ExpressionStmt(Box::new(ExpressionStmt {
            expression,
        })))
    }

    fn expression(&self) -> Result<Expression, ParseError> {
        return self.comma();
    }

    fn comma(&self) -> Result<Expression, ParseError> {
        let mut left = self.assignment()?;

        while self.matches(&[TokenType::Comma]) {
            let right = self.assignment()?;
            left = Expression::Comma(Box::new(Comma { left, right }))
        }

        Ok(left)
    }

    fn assignment(&self) -> Result<Expression, ParseError> {
        let variable = self.ternary()?;

        if self.matches(&[TokenType::Equal]) {
            match variable {
                Expression::Variable(variable) => {
                    let expression = self.assignment()?;

                    Ok(Expression::Assignment(Box::new(Assignment {
                        name: variable.name,
                        expression,
                    })))
                }
                _ => Err(Self::error(
                    self.previous(),
                    "Invalid left hand side in assignment",
                )),
            }
        } else {
            Ok(variable)
        }
    }

    pub fn ternary(&self) -> Result<Expression, ParseError> {
        let mut condition = self.or()?;

        if self.matches(&[TokenType::Question]) {
            let truth = self.expression()?;

            self.consume(&TokenType::Colon, "Expect a ':' a falsy expression")?;

            let falsy = self.expression()?;

            condition = Expression::Ternary(Box::new(Ternary {
                condition,
                truth,
                falsy,
            }))
        }

        Ok(condition)
    }

    pub fn or(&self) -> Result<Expression, ParseError> {
        let mut left = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            left = Expression::Logical(Box::new(Logical {
                operator,
                left,
                right,
            }))
        }

        Ok(left)
    }

    pub fn and(&self) -> Result<Expression, ParseError> {
        let mut left = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            left = Expression::Logical(Box::new(Logical {
                operator,
                left,
                right,
            }))
        }

        Ok(left)
    }

    fn equality(&self) -> Result<Expression, ParseError> {
        let mut left = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn comparison(&self) -> Result<Expression, ParseError> {
        let mut left = self.term()?;

        while self.matches(&[
            TokenType::LessEqual,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::Greater,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn term(&self) -> Result<Expression, ParseError> {
        let mut left = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn factor(&self) -> Result<Expression, ParseError> {
        let mut left = self.unary()?;

        while self.matches(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            left = Expression::Binary(Box::new(Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn unary(&self) -> Result<Expression, ParseError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            Ok(Expression::Unary(Box::new(Unary { operator, right })))
        } else {
            self.call()
        }
    }

    fn finish_call(&self, callee: Expression) -> Result<Expression, ParseError> {
        let mut arguments = vec![];

        if !self.check(&TokenType::RightParenthesis) {
            loop {
                if arguments.len() >= 255 {
                    Self::error(self.peek(), "Can't have more than 255 arguments.");
                }

                arguments.push(self.assignment()?);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self
            .consume(&TokenType::RightParenthesis, "Expect ')' after arguments")?
            .clone();

        Ok(Expression::Call(Box::new(Call {
            arguments,
            callee,
            paren,
        })))
    }

    fn call(&self) -> Result<Expression, ParseError> {
        let mut callee = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParenthesis]) {
                callee = self.finish_call(callee)?;
            } else {
                break;
            }
        }

        Ok(callee)
    }

    fn primary(&self) -> Result<Expression, ParseError> {
        if self.matches(&[TokenType::NumberLiteral]) {
            let number = self.previous().literal.as_ref().unwrap();

            if let LiteralType::Number(value) = number {
                return Ok(Expression::Literal(Box::new(Literal {
                    value: Object::Number(*value),
                })));
            }
        }

        if self.matches(&[TokenType::StringLiteral]) {
            let string = self.previous().literal.as_ref().unwrap();

            if let LiteralType::String(value) = string {
                return Ok(Expression::Literal(Box::new(Literal {
                    value: Object::String(String::from(value)),
                })));
            }
        }

        if self.matches(&[TokenType::False]) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: Object::Boolean(false),
            })));
        }

        if self.matches(&[TokenType::True]) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: Object::Boolean(true),
            })));
        }

        if self.matches(&[TokenType::Undefined]) {
            return Ok(Expression::Literal(Box::new(Literal {
                value: Object::Undefined,
            })));
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expression::Variable(Box::new(Variable {
                name: self.previous().clone(),
            })));
        }

        if self.matches(&[TokenType::LeftParenthesis]) {
            let expression = self.expression()?;

            self.consume(&TokenType::RightParenthesis, "Expect a ')'")?;

            return Ok(Expression::Grouping(Box::new(Grouping { expression })));
        }

        if self.matches(&[
            TokenType::EqualEqual,
            TokenType::BangEqual,
            TokenType::LessEqual,
            TokenType::Less,
            TokenType::GreaterEqual,
            TokenType::Greater,
            TokenType::Plus,
            TokenType::Star,
            TokenType::Slash,
        ]) {
            Self::error(
                &self.peek(),
                "Expect expression on left side of binary expression",
            );

            return self.expression();
        }

        Err(Self::error(&self.peek(), "Expect an expression"))
    }

    fn matches(&self, tokens: &[TokenType]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn consume(&self, token: &TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token) {
            return Ok(self.advance());
        }

        Err(Self::error(self.peek(), message))
    }

    fn check(&self, token: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };

        token == &self.peek().token_type
    }

    fn advance(&self) -> &Token {
        if !self.is_at_end() {
            *self.current.borrow_mut() += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[*self.current.borrow()]
    }

    fn previous(&self) -> &Token {
        &self.tokens[*self.current.borrow() - 1]
    }

    fn error(token: &Token, message: &str) -> ParseError {
        Lib::error_two(token, message);

        ParseError
    }

    fn synchronize(&self) {
        *self.loop_depth.borrow_mut() = 0;
        *self.in_function.borrow_mut() = false;
        self.advance();

        while !self.is_at_end() {
            match self.peek().token_type {
                TokenType::Class
                | TokenType::If
                | TokenType::While
                | TokenType::For
                | TokenType::Var
                | TokenType::Function
                | TokenType::Return => return,
                TokenType::SemiColon => {
                    self.advance();

                    return;
                }
                _ => self.advance(),
            };
        }
    }
}
