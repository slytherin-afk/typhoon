use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    environment::Environment,
    expression::{
        binary::Binary, comma::Comma, grouping::Grouping, literal::Literal, ternary::Ternary,
        unary::Unary, variable::Variable, Expression,
    },
    object::Object,
    scanner::{token::Token, token_type::TokenType},
    stmt::{
        block_stmt::BlockStmt, expression_stmt::ExpressionStmt, print_stmt::PrintStmt,
        variable_stmt::VariableStmt, Stmt,
    },
    Typhoon,
};

use super::{ExpressionVisitor, StmtVisitor};

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn interpret(
        stmts: &mut Vec<Stmt>,
        typhoon: &mut Typhoon,
        environment: Rc<RefCell<Environment>>,
    ) {
        let mut interpreter = Self { environment };

        for stmt in stmts {
            if let Err(e) = interpreter.execute(stmt) {
                typhoon.runtime_error(&e);
            }
        }
    }

    fn evaluate(&mut self, expr: &mut Expression) -> Result<Object, RuntimeError> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &mut Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        block: &mut BlockStmt,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        let previous_env = Rc::clone(&self.environment);

        self.environment = env;

        let mut error = None;

        for stmt in &mut block.stmts {
            if let Err(err) = self.execute(stmt) {
                error = Some(err);
                break;
            }
        }

        self.environment = previous_env;

        if let Some(err) = error {
            Err(err)
        } else {
            Ok(())
        }
    }
}

impl Interpreter {
    fn handle_addition(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, RuntimeError> {
        Self::validate_addition_operands(&left, &right, &operator)?;

        match (left, right) {
            (Object::String(l), Object::String(r)) => return Ok(Object::String(format!("{l}{r}"))),
            _ => {
                let result = Self::to_number(left) + Self::to_number(right);

                return Ok(Object::Number(result));
            }
        };
    }

    fn handle_arithmetic(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, RuntimeError> {
        Self::validate_arithmetic_operands(left, right, operator)?;

        let left_number = Self::to_number(left);
        let right_number = Self::to_number(right);
        let value = match operator.token_type {
            TokenType::Minus => left_number - right_number,
            TokenType::Star => left_number * right_number,
            TokenType::Slash => {
                if right_number == 0.0 {
                    return Err(RuntimeError::new(
                        operator.clone(),
                        "Not divisible by Zero".to_string(),
                    ));
                } else {
                    left_number / right_number
                }
            }
            _ => unreachable!(),
        };

        Ok(Object::Number(value))
    }

    fn handle_comparison(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, RuntimeError> {
        Self::validate_addition_operands(&left, &right, &operator)?;

        let value = match (left, right) {
            (Object::String(l), Object::String(r)) => match operator.token_type {
                TokenType::Greater => l > r,
                TokenType::GreaterEqual => l >= r,
                TokenType::Less => l < r,
                TokenType::LessEqual => l <= r,
                _ => unreachable!(),
            },
            _ => {
                let l = Self::to_number(left);
                let r = Self::to_number(right);

                match operator.token_type {
                    TokenType::Greater => l > r,
                    TokenType::GreaterEqual => l >= r,
                    TokenType::Less => l < r,
                    TokenType::LessEqual => l <= r,
                    _ => unreachable!(),
                }
            }
        };

        Ok(Object::Boolean(value))
    }

    fn validate_addition_operands(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<(), RuntimeError> {
        match (left, right) {
            (Object::Number(_), Object::Number(_))
            | (Object::Number(_), Object::Boolean(_))
            | (Object::Boolean(_), Object::Number(_))
            | (Object::String(_), Object::String(_)) => Ok(()),
            _ => Err(RuntimeError::new(
                operator.clone(),
                "Operands must be (numbers or booleans) or two strings".to_string(),
            )),
        }
    }

    fn validate_arithmetic_operands(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<(), RuntimeError> {
        match (left, right) {
            (Object::Number(_), Object::Number(_))
            | (Object::Number(_), Object::Boolean(_))
            | (Object::Boolean(_), Object::Number(_)) => Ok(()),
            _ => Err(RuntimeError::new(
                operator.clone(),
                "Operands must be numbers or booleans".to_string(),
            )),
        }
    }

    fn to_number(value: &Object) -> f64 {
        match value {
            Object::Number(n) => *n,
            Object::Boolean(b) => Self::bool_to_number(*b),
            _ => unreachable!(),
        }
    }

    fn bool_to_number(boolean: bool) -> f64 {
        if boolean {
            1.0
        } else {
            0.0
        }
    }

    fn is_truthy(literal: &Object) -> bool {
        match literal {
            Object::Undefined => false,
            Object::Number(number) => *number != 0.0,
            Object::String(string) => !string.is_empty(),
            Object::Boolean(boolean) => *boolean,
        }
    }
}

impl ExpressionVisitor for Interpreter {
    type Item = Result<Object, RuntimeError>;

    fn visit_comma(&mut self, expr: &mut Comma) -> Self::Item {
        self.evaluate(&mut expr.left)?;
        self.evaluate(&mut expr.right)
    }

    fn visit_ternary(&mut self, expr: &mut Ternary) -> Self::Item {
        let condition = self.evaluate(&mut expr.condition)?;

        if Self::is_truthy(&condition) {
            self.evaluate(&mut expr.truth)
        } else {
            self.evaluate(&mut expr.falsy)
        }
    }

    fn visit_binary(&mut self, expr: &mut Binary) -> Self::Item {
        let left = self.evaluate(&mut expr.left)?;
        let right = self.evaluate(&mut expr.right)?;

        match &expr.operator.token_type {
            TokenType::Plus => Self::handle_addition(&left, &right, &expr.operator),
            TokenType::Minus | TokenType::Star | TokenType::Slash => {
                Self::handle_arithmetic(&left, &right, &expr.operator)
            }
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => Self::handle_comparison(&left, &right, &expr.operator),
            TokenType::BangEqual => Ok(Object::Boolean(left != right)),
            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
            _ => unreachable!(),
        }
    }

    fn visit_unary(&mut self, expr: &mut Unary) -> Self::Item {
        let literal = self.evaluate(&mut expr.right)?;
        let literal = match expr.operator.token_type {
            TokenType::Bang => Object::Boolean(!Self::is_truthy(&literal)),
            TokenType::Minus => {
                let literal = match literal {
                    Object::Number(number) => number,
                    Object::Boolean(boolean) => Self::bool_to_number(boolean),
                    _ => {
                        return Err(RuntimeError::new(
                            expr.operator.clone(),
                            "Unary minus requires number or boolean operand".to_string(),
                        ))
                    }
                };

                Object::Number(-literal)
            }
            _ => unreachable!(),
        };

        Ok(literal)
    }

    fn visit_grouping(&mut self, expr: &mut Grouping) -> Self::Item {
        self.evaluate(&mut expr.expression)
    }

    fn visit_literal(&mut self, expr: &mut Literal) -> Self::Item {
        Ok(expr.value.clone())
    }

    fn visit_variable(&mut self, expr: &mut Variable) -> Self::Item {
        self.environment.borrow().get(&expr.name)
    }

    fn visit_assignment(
        &mut self,
        expr: &mut crate::expression::assignment::Assignment,
    ) -> Self::Item {
        let value = self.evaluate(&mut expr.expression)?;

        self.environment
            .borrow_mut()
            .assign(expr.name, value.clone())?;

        Ok(value)
    }
}

impl StmtVisitor for Interpreter {
    type Item = Result<(), RuntimeError>;

    fn visit_print_stmt(&mut self, stmt: &mut PrintStmt) -> Self::Item {
        let value = self.evaluate(&mut stmt.expression)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_exit_stmt(&mut self, stmt: &mut crate::stmt::exit_stmt::ExitStmt) -> Self::Item {
        let exit_code = match &mut stmt.expression {
            Some(expression) => {
                let value = self.evaluate(expression)?;
                match value {
                    Object::Number(_) | Object::Boolean(_) => Self::to_number(&value) as i32,
                    _ => {
                        println!("{value}");

                        1
                    }
                }
            }
            None => 0,
        };

        exit(exit_code);
    }

    fn visit_expression_stmt(&mut self, stmt: &mut ExpressionStmt) -> Self::Item {
        let value = self.evaluate(&mut stmt.expression)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_variable_stmt(&mut self, stmt: &mut VariableStmt) -> Self::Item {
        let value = if let Some(expr) = &mut stmt.initializer {
            self.evaluate(expr)?
        } else {
            Object::Undefined
        };

        for name in &stmt.names {
            self.environment
                .borrow_mut()
                .define(name.lexeme.to_string(), value.clone());
        }

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &mut BlockStmt) -> Self::Item {
        self.execute_block(
            stmt,
            Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                &self.environment,
            ))))),
        )?;

        Ok(())
    }
}
