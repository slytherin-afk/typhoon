use crate::{
    errors::SyntaxError,
    expr::{self, Expr, Super},
    literal_type::LiteralType,
    object::Object,
    stmt::{self, Stmt},
    token::Token,
    token_type::TokenType,
    Lib,
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

    fn stmt(&mut self) -> Result<Stmt, SyntaxError> {
        if self.matches(&[TokenType::SemiColon]) {
            Ok(Stmt::Empty)
        } else if self.matches(&[TokenType::Print]) {
            self.print_stmt()
        } else if self.matches(&[TokenType::LeftBraces]) {
            Ok(Stmt::Block(Box::new(self.block_stmt()?)))
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

    fn expr_stmt(&mut self) -> Result<Stmt, SyntaxError> {
        let value = self.expression()?;

        self.consume(
            &TokenType::SemiColon,
            "Expect a ';' at the end of expression",
        )?;

        Ok(Stmt::Expression(Box::new(value)))
    }

    fn print_stmt(&mut self) -> Result<Stmt, SyntaxError> {
        let value = self.expression()?;

        self.consume(&TokenType::SemiColon, "Expect a ';' at the end of print")?;

        Ok(Stmt::Print(Box::new(value)))
    }

    fn variable_stmt(&mut self) -> Result<Stmt, SyntaxError> {
        let mut stmts = vec![];
        let name = self
            .consume(&TokenType::Identifier, "Expect an identifier")?
            .clone();
        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.assignment()?)
        } else {
            None
        };

        stmts.push(stmt::VariableDeclaration { name, initializer });

        while self.matches(&[TokenType::Comma]) {
            let name = self
                .consume(&TokenType::Identifier, "Expect an identifier")?
                .clone();
            let initializer = if self.matches(&[TokenType::Equal]) {
                Some(self.assignment()?)
            } else {
                None
            };

            stmts.push(stmt::VariableDeclaration { name, initializer });
        }

        self.consume(
            &TokenType::SemiColon,
            "Expect a ';' at the end of variable declaration",
        )?;

        Ok(Stmt::Variable(Box::new(stmts)))
    }

    fn block_stmt(&mut self) -> Result<Vec<Stmt>, SyntaxError> {
        let mut stmts = vec![];

        while !self.check(&TokenType::RightBraces) && !self.is_at_end() {
            if let Some(stmt) = self.declaration_stmt() {
                stmts.push(stmt);
            }
        }

        self.consume(&TokenType::RightBraces, "Expect a '}' at the end of block")?;

        Ok(stmts)
    }

    fn if_stmt(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume(&TokenType::LeftParenthesis, "Expect a '(' after if")?;

        let condition = self.expression()?;

        self.consume(&TokenType::RightParenthesis, "Expect a ')' before if body")?;

        let truth = self.stmt()?;
        let falsy = if self.matches(&[TokenType::Else]) {
            Some(self.stmt()?)
        } else {
            None
        };

        Ok(Stmt::If(Box::new(stmt::If {
            condition,
            truth,
            falsy,
        })))
    }

    fn while_stmt(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume(&TokenType::LeftParenthesis, "Expect a '(' after while")?;

        let condition = self.expression()?;

        self.consume(
            &TokenType::RightParenthesis,
            "Expect a ')' before while body",
        )?;

        let body = self.stmt()?;

        Ok(Stmt::While(Box::new(stmt::While { condition, body })))
    }

    fn for_stmt(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume(&TokenType::LeftParenthesis, "Expect a '(' after for")?;

        let initializer = if self.matches(&[TokenType::SemiColon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.variable_stmt()?)
        } else {
            Some(self.expr_stmt()?)
        };

        let condition = if self.check(&TokenType::SemiColon) {
            Expr::Literal(Box::new(Object::Boolean(true)))
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
            body = Stmt::Block(Box::new(vec![body, Stmt::Expression(Box::new(value))]));
        }

        body = Stmt::While(Box::new(stmt::While { condition, body }));

        if let Some(initializer) = initializer {
            body = Stmt::Block(Box::new(vec![initializer, body]));
        }

        Ok(body)
    }

    fn loop_control(&mut self) -> Result<Stmt, SyntaxError> {
        let token = self.previous().clone();

        let result = if token.token_type == TokenType::Continue {
            Ok(Stmt::Continue(token))
        } else {
            Ok(Stmt::Break(token))
        };

        self.consume(&TokenType::SemiColon, "Expected ';' at end of loop control")?;

        result
    }

    fn function_stmt(&mut self, kind: &str) -> Result<Stmt, SyntaxError> {
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

        Ok(Stmt::Function(Box::new(stmt::Function {
            name,
            params,
            body,
        })))
    }

    fn return_stmt(&mut self) -> Result<Stmt, SyntaxError> {
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

        Ok(Stmt::Return(Box::new(stmt::Return { keyword, value })))
    }

    fn class_stmt(&mut self) -> Result<Stmt, SyntaxError> {
        let name = self
            .consume(&TokenType::Identifier, "Expected an identifier after class")?
            .clone();

        let super_class = if self.matches(&[TokenType::Less]) {
            Some(Expr::Variable(Box::new(
                self.consume(&TokenType::Identifier, "Expected a super class name")?
                    .clone(),
            )))
        } else {
            None
        };

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

        Ok(Stmt::Class(Box::new(stmt::Class {
            name,
            super_class,
            methods,
            statics,
        })))
    }

    fn expression(&mut self) -> Result<Expr, SyntaxError> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = self.assignment()?;

        while self.matches(&[TokenType::Comma]) {
            let right = self.assignment()?;
            left = Expr::Comma(Box::new(expr::Comma { left, right }))
        }

        Ok(left)
    }

    fn lambda(&mut self) -> Result<Expr, SyntaxError> {
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

        Ok(Expr::Lambda(Box::new(expr::Lambda { name, params, body })))
    }

    fn assignment(&mut self) -> Result<Expr, SyntaxError> {
        if self.matches(&[TokenType::Function]) {
            return self.lambda();
        }

        let variable = self.ternary()?;

        if self.matches(&[TokenType::Equal]) {
            match variable {
                Expr::Variable(variable) => {
                    let value = self.assignment()?;

                    Ok(Expr::Assignment(Box::new(expr::Assignment {
                        name: *variable,
                        value,
                    })))
                }
                Expr::Get(get) => {
                    let value = self.assignment()?;

                    Ok(Expr::Set(Box::new(expr::Set {
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

    fn ternary(&mut self) -> Result<Expr, SyntaxError> {
        let mut condition = self.or()?;

        if self.matches(&[TokenType::Question]) {
            let truth = self.expression()?;

            self.consume(&TokenType::Colon, "Expect a ':' a falsy expression")?;

            let falsy = self.expression()?;

            condition = Expr::Ternary(Box::new(expr::Ternary {
                condition,
                truth,
                falsy,
            }))
        }

        Ok(condition)
    }

    fn or(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            left = Expr::Logical(Box::new(expr::Logical {
                operator,
                left,
                right,
            }))
        }

        Ok(left)
    }

    fn and(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            left = Expr::Logical(Box::new(expr::Logical {
                operator,
                left,
                right,
            }))
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            left = Expr::Binary(Box::new(expr::Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = self.term()?;

        while self.matches(&[
            TokenType::LessEqual,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::Greater,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            left = Expr::Binary(Box::new(expr::Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            left = Expr::Binary(Box::new(expr::Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = self.unary()?;

        while self.matches(&[TokenType::Star, TokenType::Slash, TokenType::Percentage]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            left = Expr::Binary(Box::new(expr::Binary {
                left,
                operator,
                right,
            }))
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, SyntaxError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            Ok(Expr::Unary(Box::new(expr::Unary { operator, right })))
        } else {
            self.call()
        }
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, SyntaxError> {
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

        Ok(Expr::Call(Box::new(expr::Call {
            arguments,
            callee,
            paren,
        })))
    }

    fn call(&mut self) -> Result<Expr, SyntaxError> {
        let mut callee = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParenthesis]) {
                callee = self.finish_call(callee)?;
            } else if self.matches(&[TokenType::Dot]) {
                let name = self
                    .consume(&TokenType::Identifier, "Expect property name")?
                    .clone();
                callee = Expr::Get(Box::new(expr::Get {
                    object: callee,
                    name,
                }))
            } else {
                break;
            }
        }

        Ok(callee)
    }

    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        if self.matches(&[TokenType::LeftParenthesis]) {
            let expression = self.expression()?;

            self.consume(&TokenType::RightParenthesis, "Expect a ')'")?;

            return Ok(Expr::Grouping(Box::new(expression)));
        }

        if self.matches(&[TokenType::This]) {
            return Ok(Expr::This(Box::new(self.previous().clone())));
        }

        if self.matches(&[TokenType::Super]) {
            let keyword = self.previous().clone();

            self.consume(&TokenType::Dot, "Expect a '.' after 'super'")?;

            let method = self
                .consume(&TokenType::Identifier, "Expect an super class method name")?
                .clone();

            return Ok(Expr::Super(Box::new(Super { keyword, method })));
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(Box::new(self.previous().clone())));
        }

        if self.matches(&[TokenType::Undefined]) {
            return Ok(Expr::Literal(Box::new(Object::Undefined)));
        }

        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal(Box::new(Object::Boolean(false))));
        }

        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal(Box::new(Object::Boolean(true))));
        }

        if self.matches(&[TokenType::NumberLiteral]) {
            let number = self.previous().literal.as_ref().unwrap();

            if let LiteralType::Number(value) = number {
                return Ok(Expr::Literal(Box::new(Object::Number(*value))));
            }
        }

        if self.matches(&[TokenType::StringLiteral]) {
            let string = self.previous().literal.as_ref().unwrap();

            if let LiteralType::String(value) = string {
                return Ok(Expr::Literal(Box::new(Object::String(String::from(value)))));
            }
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

    fn consume(&mut self, token: &TokenType, message: &str) -> Result<&Token, SyntaxError> {
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

    fn error(token: &Token, message: &str) -> SyntaxError {
        Lib::error_token(token, message);

        SyntaxError
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
