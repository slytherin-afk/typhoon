pub mod operations;

use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    environment::Environment,
    expression::{
        assignment::Assignment, binary::Binary, comma::Comma, grouping::Grouping, literal::Literal,
        logical::Logical, ternary::Ternary, unary::Unary, variable::Variable, Expression,
    },
    object::Object,
    scanner::{token::Token, token_type::TokenType},
    stmt::{
        block_stmt::BlockStmt, exit_stmt::ExitStmt, expression_stmt::ExpressionStmt,
        if_stmt::IfStmt, print_stmt::PrintStmt, variable_stmt::VariableStmt, while_stmt::WhileStmt,
        Stmt,
    },
    Lib,
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

pub struct BreakException;

pub struct ContinueException;

pub enum Exception {
    RuntimeError(RuntimeError),
    BreakException,
    ContinueException,
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn interpret(stmts: &mut Vec<Stmt>, lib: &mut Lib, environment: Rc<RefCell<Environment>>) {
        let mut interpreter = Self { environment };

        for stmt in stmts {
            if let Err(e) = interpreter.execute(stmt) {
                match e {
                    Exception::RuntimeError(runtime_error) => lib.runtime_error(&runtime_error),
                    _ => unreachable!(),
                };
            }
        }
    }

    fn evaluate(&mut self, expr: &mut Expression) -> Result<Object, RuntimeError> {
        expr.accept(self)
    }

    fn evaluate_and_map_error(&mut self, expr: &mut Expression) -> Result<Object, Exception> {
        self.evaluate(expr).map_err(|e| Exception::RuntimeError(e))
    }

    fn execute(&mut self, stmt: &mut Stmt) -> Result<(), Exception> {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        block: &mut BlockStmt,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(), Exception> {
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

impl ExpressionVisitor for Interpreter {
    type Item = Result<Object, RuntimeError>;

    fn visit_comma(&mut self, expr: &mut Comma) -> Self::Item {
        self.evaluate(&mut expr.left)?;
        self.evaluate(&mut expr.right)
    }

    fn visit_ternary(&mut self, expr: &mut Ternary) -> Self::Item {
        let condition = self.evaluate(&mut expr.condition)?;

        if operations::is_truthy(&condition) {
            self.evaluate(&mut expr.truth)
        } else {
            self.evaluate(&mut expr.falsy)
        }
    }

    fn visit_binary(&mut self, expr: &mut Binary) -> Self::Item {
        let left = self.evaluate(&mut expr.left)?;
        let right = self.evaluate(&mut expr.right)?;

        match &expr.operator.token_type {
            TokenType::Plus => operations::handle_addition(&left, &right, &expr.operator),
            TokenType::Minus | TokenType::Star | TokenType::Slash => {
                operations::handle_arithmetic(&left, &right, &expr.operator)
            }
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => operations::handle_comparison(&left, &right, &expr.operator),
            TokenType::BangEqual => Ok(Object::Boolean(left != right)),
            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
            _ => unreachable!(),
        }
    }

    fn visit_unary(&mut self, expr: &mut Unary) -> Self::Item {
        let literal = self.evaluate(&mut expr.right)?;
        let literal = match expr.operator.token_type {
            TokenType::Bang => Object::Boolean(!operations::is_truthy(&literal)),
            TokenType::Minus => {
                let literal = match literal {
                    Object::Number(number) => number,
                    Object::Boolean(boolean) => operations::bool_to_number(boolean),
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

    fn visit_assignment(&mut self, expr: &mut Assignment) -> Self::Item {
        let value = self.evaluate(&mut expr.expression)?;

        self.environment
            .borrow_mut()
            .assign(expr.name, value.clone())?;

        Ok(value)
    }

    fn visit_logical(&mut self, expr: &mut Logical) -> Self::Item {
        let left = self.evaluate(&mut expr.left)?;
        let is_truthy = operations::is_truthy(&left);
        let value = match expr.operator.token_type {
            TokenType::And => {
                if is_truthy {
                    self.evaluate(&mut expr.right)?
                } else {
                    left
                }
            }
            TokenType::Or => {
                if is_truthy {
                    left
                } else {
                    self.evaluate(&mut expr.right)?
                }
            }
            _ => unreachable!(),
        };

        Ok(value)
    }
}

impl StmtVisitor for Interpreter {
    type Item = Result<(), Exception>;

    fn visit_print_stmt(&mut self, stmt: &mut PrintStmt) -> Self::Item {
        let value = self.evaluate_and_map_error(&mut stmt.expression)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_exit_stmt(&mut self, stmt: &mut ExitStmt) -> Self::Item {
        let exit_code = match &mut stmt.expression {
            Some(expression) => {
                let value = self.evaluate_and_map_error(expression)?;
                match value {
                    Object::Number(_) | Object::Boolean(_) => operations::to_number(&value) as i32,
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
        let value = self.evaluate_and_map_error(&mut stmt.expression)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_variable_stmt(&mut self, stmt: &mut VariableStmt) -> Self::Item {
        for var in &mut stmt.variables {
            let value = if let Some(expr) = &mut var.initializer {
                self.evaluate_and_map_error(expr)?
            } else {
                Object::Undefined
            };

            self.environment
                .borrow_mut()
                .define(var.name.lexeme.to_string(), value.clone());
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

    fn visit_if_stmt(&mut self, stmt: &mut IfStmt) -> Self::Item {
        let condition = self.evaluate_and_map_error(&mut stmt.condition)?;

        if operations::is_truthy(&condition) {
            self.execute(&mut stmt.truth)?;
        } else if let Some(falsy_stmt) = &mut stmt.falsy {
            self.execute(falsy_stmt)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &mut WhileStmt) -> Self::Item {
        while operations::is_truthy(&self.evaluate_and_map_error(&mut stmt.condition)?) {
            let result = self.execute(&mut stmt.body);

            if let Err(e) = &result {
                match e {
                    Exception::BreakException => break,
                    Exception::ContinueException => continue,
                    _ => result?,
                }
            }
        }

        Ok(())
    }

    fn visit_empty_stmt(&mut self) -> Self::Item {
        Ok(())
    }

    fn visit_continue_stmt(&mut self) -> Self::Item {
        Err(Exception::ContinueException)
    }

    fn visit_break_stmt(&mut self) -> Self::Item {
        Err(Exception::BreakException)
    }
}
