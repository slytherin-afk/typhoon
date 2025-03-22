use crate::{
    errors::ParseError,
    expression::{
        Assignment, Binary, Call, Comma, Expression, Get, Lambda, Logical, Set, Ternary, Unary,
    },
    stmt::{ClassStmt, FunctionStmt, IfStmt, ReturnStmt, Stmt, VariableDeclaration, WhileStmt},
    Lib, LiteralType, Object, Token, TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration_stmt() {
                statements.push(stmt)
            };
        }

        statements
    }

    fn declaration_stmt(&mut self) -> Option<Stmt> {
        let stmt = if self.matches(&[TokenType::Var]) {
            self.variable_stmt()
        } else {
            self.stmt()
        };

        if let Err(_) = stmt {
            self.synchronize();
        }

        stmt.ok()
    }

    fn stmt(&mut self) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::SemiColon]) {
            Ok(Stmt::EmptyStmt)
        } else if self.matches(&[TokenType::Print]) {
            self.print_stmt()
        } else if self.matches(&[TokenType::LeftBraces]) {
            Ok(Stmt::BlockStmt(Box::new(self.block_stmt()?)))
        } else if self.matches(&[TokenType::If]) {
            self.if_stmt()
        } else if self.matches(&[TokenType::While]) {
            self.while_stmt()
        } else if self.matches(&[TokenType::For]) {
            self.for_stmt()
        } else if self.matches(&[TokenType::Break, TokenType::Continue]) {
            self.loop_control()
        } else if self.matches(&[TokenType::Function]) {
            self.function_stmt("function")
        } else if self.matches(&[TokenType::Return]) {
            self.return_stmt()
        } else if self.matches(&[TokenType::Class]) {
            self.class_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;

        self.consume(
            &TokenType::SemiColon,
            "Expect a ';' at the end of expression",
        )?;

        Ok(Stmt::ExpressionStmt(Box::new(value)))
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;

        self.consume(&TokenType::SemiColon, "Expect a ';' at the end of print")?;

        Ok(Stmt::PrintStmt(Box::new(value)))
    }

    fn variable_stmt(&mut self) -> Result<Stmt, ParseError> {
        let mut stmts = vec![];
        let name = self
            .consume(&TokenType::Identifier, "Expect an identifier")?
            .clone();
        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.assignment()?)
        } else {
            None
        };

        stmts.push(VariableDeclaration { name, initializer });

        while self.matches(&[TokenType::Comma]) {
            let name = self
                .consume(&TokenType::Identifier, "Expect an identifier")?
                .clone();
            let initializer = if self.matches(&[TokenType::Equal]) {
                Some(self.assignment()?)
            } else {
                None
            };

            stmts.push(VariableDeclaration { name, initializer });
        }

        self.consume(
            &TokenType::SemiColon,
            "Expect a ';' at the end of variable declaration",
        )?;

        Ok(Stmt::VariableStmt(Box::new(stmts)))
    }

    fn block_stmt(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = vec![];

        while !self.check(&TokenType::RightBraces) && !self.is_at_end() {
            if let Some(stmt) = self.declaration_stmt() {
                stmts.push(stmt);
            }
        }

        self.consume(&TokenType::RightBraces, "Expect a '}' at the end of block")?;

        Ok(stmts)
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
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

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParenthesis, "Expect a '(' after while")?;

        let condition = self.expression()?;

        self.consume(
            &TokenType::RightParenthesis,
            "Expect a ')' before while body",
        )?;

        let body = self.stmt()?;

        Ok(Stmt::WhileStmt(Box::new(WhileStmt { condition, body })))
    }

    fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParenthesis, "Expect a '(' after for")?;

        let initializer = if self.matches(&[TokenType::SemiColon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.variable_stmt()?)
        } else {
            Some(self.expr_stmt()?)
        };

        let condition = if self.check(&TokenType::SemiColon) {
            Expression::Literal(Box::new(Object::Boolean(true)))
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

        let mut body = self.stmt()?;

        if let Some(value) = increment {
            body = Stmt::BlockStmt(Box::new(vec![body, Stmt::ExpressionStmt(Box::new(value))]));
        }

        body = Stmt::WhileStmt(Box::new(WhileStmt { condition, body }));

        if let Some(initializer) = initializer {
            body = Stmt::BlockStmt(Box::new(vec![initializer, body]));
        }

        Ok(body)
    }

    fn loop_control(&mut self) -> Result<Stmt, ParseError> {
        let token = self.previous().clone();

        let result = if token.token_type == TokenType::Continue {
            Ok(Stmt::ContinueStmt(token))
        } else {
            Ok(Stmt::BreakStmt(token))
        };

        self.consume(&TokenType::SemiColon, "Expected ';' at end of loop control")?;

        result
    }

    fn function_stmt(&mut self, kind: &str) -> Result<Stmt, ParseError> {
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

    fn return_stmt(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        let value = if !self.check(&TokenType::SemiColon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            &TokenType::SemiColon,
            &format!("Expect ';' at the end of return"),
        )?;

        Ok(Stmt::ReturnStmt(Box::new(ReturnStmt { keyword, value })))
    }

    fn class_stmt(&mut self) -> Result<Stmt, ParseError> {
        let name = self
            .consume(&TokenType::Identifier, "Expected an identifier after class")?
            .clone();

        self.consume(&TokenType::LeftBraces, "Expected '{' after class body")?;

        let mut methods = vec![];
        let mut statics = vec![];

        while !self.check(&TokenType::RightBraces) {
            if self.matches(&[TokenType::Class]) {
                statics.push(self.function_stmt("static")?);
            } else {
                methods.push(self.function_stmt("method")?);
            }
        }

        self.consume(
            &TokenType::RightBraces,
            "Expected '}' at the end of class body",
        )?;

        Ok(Stmt::ClassStmt(Box::new(ClassStmt {
            name,
            methods,
            statics,
        })))
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.assignment()?;

        while self.matches(&[TokenType::Comma]) {
            let right = self.assignment()?;
            left = Expression::Comma(Box::new(Comma { left, right }))
        }

        Ok(left)
    }

    fn lambda(&mut self) -> Result<Expression, ParseError> {
        let name = self.previous().clone();

        self.consume(
            &TokenType::LeftParenthesis,
            &format!("Expect '(' after anonymous function name"),
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
                        &format!("Expect identifier after anonymous function name"),
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
            &format!("Expect ')' after anonymous function params"),
        )?;

        self.consume(
            &TokenType::LeftBraces,
            &format!("Expect '{{' after anonymous function params"),
        )?;

        let body = self.block_stmt()?;

        Ok(Expression::Lambda(Box::new(Lambda { name, params, body })))
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        if self.matches(&[TokenType::Function]) {
            return self.lambda();
        }

        let variable = self.ternary()?;

        if self.matches(&[TokenType::Equal]) {
            match variable {
                Expression::Variable(variable) => {
                    let value = self.assignment()?;

                    Ok(Expression::Assignment(Box::new(Assignment {
                        name: *variable,
                        value,
                    })))
                }
                Expression::Get(get) => {
                    let value = self.assignment()?;

                    Ok(Expression::Set(Box::new(Set {
                        object: get.object,
                        name: get.name,
                        value,
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

    fn ternary(&mut self) -> Result<Expression, ParseError> {
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

    fn or(&mut self) -> Result<Expression, ParseError> {
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

    fn and(&mut self) -> Result<Expression, ParseError> {
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

    fn equality(&mut self) -> Result<Expression, ParseError> {
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

    fn comparison(&mut self) -> Result<Expression, ParseError> {
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

    fn term(&mut self) -> Result<Expression, ParseError> {
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

    fn factor(&mut self) -> Result<Expression, ParseError> {
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

    fn unary(&mut self) -> Result<Expression, ParseError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            Ok(Expression::Unary(Box::new(Unary { operator, right })))
        } else {
            self.call()
        }
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, ParseError> {
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

    fn call(&mut self) -> Result<Expression, ParseError> {
        let mut callee = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParenthesis]) {
                callee = self.finish_call(callee)?;
            } else if self.matches(&[TokenType::Dot]) {
                let name = self
                    .consume(&TokenType::Identifier, "Expect property name")?
                    .clone();
                callee = Expression::Get(Box::new(Get {
                    object: callee,
                    name,
                }))
            } else {
                break;
            }
        }

        Ok(callee)
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        if self.matches(&[TokenType::NumberLiteral]) {
            let number = self.previous().literal.as_ref().unwrap();

            if let LiteralType::Number(value) = number {
                return Ok(Expression::Literal(Box::new(Object::Number(*value))));
            }
        }

        if self.matches(&[TokenType::StringLiteral]) {
            let string = self.previous().literal.as_ref().unwrap();

            if let LiteralType::String(value) = string {
                return Ok(Expression::Literal(Box::new(Object::String(String::from(
                    value,
                )))));
            }
        }

        if self.matches(&[TokenType::False]) {
            return Ok(Expression::Literal(Box::new(Object::Boolean(false))));
        }

        if self.matches(&[TokenType::True]) {
            return Ok(Expression::Literal(Box::new(Object::Boolean(true))));
        }

        if self.matches(&[TokenType::Undefined]) {
            return Ok(Expression::Literal(Box::new(Object::Undefined)));
        }

        if self.matches(&[TokenType::This]) {
            return Ok(Expression::This(Box::new(self.previous().clone())));
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expression::Variable(Box::new(self.previous().clone())));
        }

        if self.matches(&[TokenType::LeftParenthesis]) {
            let expression = self.expression()?;

            self.consume(&TokenType::RightParenthesis, "Expect a ')'")?;

            return Ok(Expression::Grouping(Box::new(expression)));
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
                self.previous(),
                "Expect expression on left side of binary expression",
            );

            return self.expression();
        }

        Err(Self::error(&self.peek(), "Expect an expression"))
    }

    fn matches(&mut self, tokens: &[TokenType]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn consume(&mut self, token: &TokenType, message: &str) -> Result<&Token, ParseError> {
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

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(token: &Token, message: &str) -> ParseError {
        Lib::error_two(token, message);

        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Function
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return
                | TokenType::Continue
                | TokenType::Break => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}
