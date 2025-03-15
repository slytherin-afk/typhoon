pub mod callable;
pub mod function;
pub mod globals;
pub mod lambda;
pub mod operations;

use super::{ExpressionVisitor, StmtVisitor};
use crate::{
    environment::Environment,
    expression::{
        assignment::Assignment, binary::Binary, call::Call, comma::Comma, grouping::Grouping,
        lambda::Lambda, literal::Literal, logical::Logical, ternary::Ternary, unary::Unary,
        variable::Variable, Expression,
    },
    object::Object,
    scanner::{token::Token, token_type::TokenType},
    stmt::{
        block_stmt::BlockStmt, expression_stmt::ExpressionStmt, function_stmt::FunctionStmt,
        if_stmt::IfStmt, print_stmt::PrintStmt, return_stmt::ReturnStmt,
        variable_stmt::VariableStmt, while_stmt::WhileStmt, Stmt,
    },
    Lib,
};
use function::Function;
use globals::Clock;
use lambda::LambdaFunction;
use std::{cell::RefCell, rc::Rc};

pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub struct BreakException;

pub struct ContinueException;

pub enum Exception {
    RuntimeError(RuntimeError),
    ReturnException(Rc<Object>),
    BreakException,
    ContinueException,
}

pub struct Interpreter {
    _globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));

        globals.borrow_mut().define(
            String::from("clock"),
            Rc::new(Object::Callable(Rc::new(Clock))),
        );

        Self {
            environment: Rc::clone(&globals),
            _globals: globals,
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            if let Err(e) = self.execute(stmt) {
                match e {
                    Exception::RuntimeError(runtime_error) => Lib::runtime_error(&runtime_error),
                    _ => unreachable!(),
                };
            }
        }
    }

    fn evaluate(&mut self, expr: Expression) -> Result<Rc<Object>, RuntimeError> {
        expr.accept(self)
    }

    fn evaluate_and_map_error(&mut self, expr: Expression) -> Result<Rc<Object>, Exception> {
        self.evaluate(expr).map_err(|e| Exception::RuntimeError(e))
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), Exception> {
        stmt.accept(self)
    }

    fn execute_block(&mut self, stmts: Vec<Stmt>, env: Environment) -> Result<(), Exception> {
        let mut env_ref = Rc::new(RefCell::new(env));

        std::mem::swap(&mut self.environment, &mut env_ref);

        let result = stmts.into_iter().try_for_each(|stmt| self.execute(stmt));

        std::mem::swap(&mut self.environment, &mut env_ref);

        result
    }
}

impl ExpressionVisitor for Interpreter {
    type Item = Result<Rc<Object>, RuntimeError>;

    fn visit_comma(&mut self, expr: Comma) -> Self::Item {
        self.evaluate(expr.left)?;
        self.evaluate(expr.right)
    }

    fn visit_assignment(&mut self, expr: Assignment) -> Self::Item {
        let value = self.evaluate(expr.expression)?;

        self.environment
            .borrow_mut()
            .assign(expr.name, value.clone())?;

        Ok(value)
    }

    fn visit_ternary(&mut self, expr: Ternary) -> Self::Item {
        let condition = self.evaluate(expr.condition)?;

        if operations::is_truthy(&condition) {
            self.evaluate(expr.truth)
        } else {
            self.evaluate(expr.falsy)
        }
    }

    fn visit_logical(&mut self, expr: Logical) -> Self::Item {
        let left = self.evaluate(expr.left)?;
        let is_truthy = operations::is_truthy(&left);
        let value = match expr.operator.token_type {
            TokenType::And => {
                if is_truthy {
                    self.evaluate(expr.right)?
                } else {
                    left
                }
            }
            TokenType::Or => {
                if is_truthy {
                    left
                } else {
                    self.evaluate(expr.right)?
                }
            }
            _ => unreachable!(),
        };

        Ok(value)
    }

    fn visit_binary(&mut self, expr: Binary) -> Self::Item {
        let left = self.evaluate(expr.left)?;
        let right = self.evaluate(expr.right)?;
        let value = match expr.operator.token_type {
            TokenType::Plus => operations::handle_addition(&left, &right, &expr.operator),
            TokenType::Minus => operations::handle_subtraction(&left, &right, &expr.operator),
            TokenType::Star => operations::handle_multiplication(&left, &right, &expr.operator),
            TokenType::Slash => operations::handle_division(&left, &right, &expr.operator),
            TokenType::Greater => operations::handle_greater_than(&left, &right, &expr.operator),
            TokenType::GreaterEqual => {
                operations::handle_greater_than_equal(&left, &right, &expr.operator)
            }
            TokenType::Less => operations::handle_less_than(&left, &right, &expr.operator),
            TokenType::LessEqual => {
                operations::handle_less_than_equal(&left, &right, &expr.operator)
            }
            TokenType::BangEqual => Ok(Object::Boolean(left != right)),
            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
            _ => unreachable!(),
        }?;

        Ok(Rc::new(value))
    }

    fn visit_unary(&mut self, expr: Unary) -> Self::Item {
        let literal = self.evaluate(expr.right)?;
        let literal = match expr.operator.token_type {
            TokenType::Bang => Object::Boolean(!operations::is_truthy(&literal)),
            TokenType::Minus => {
                let literal = match *literal {
                    Object::Number(number) => number,
                    Object::Boolean(boolean) => operations::bool_to_number(boolean),
                    _ => {
                        return Err(RuntimeError::new(
                            expr.operator,
                            String::from("Unary minus requires number or boolean operand"),
                        ))
                    }
                };

                Object::Number(-literal)
            }
            _ => unreachable!(),
        };

        Ok(Rc::new(literal))
    }

    fn visit_call(&mut self, expr: Call) -> Self::Item {
        let callee = self.evaluate(expr.callee)?;
        let arguments = expr
            .arguments
            .into_iter()
            .map(|f| self.evaluate(f))
            .collect::<Result<Vec<_>, _>>()?;

        match &*callee {
            Object::Callable(c) => {
                let arity = c.arity();

                if arguments.len() < arity {
                    Err(RuntimeError::new(
                        expr.paren,
                        format!("Expected [{arity}] arguments got [{}]", arguments.len()),
                    ))
                } else {
                    c.call(self, arguments)
                }
            }
            _ => Err(RuntimeError::new(
                expr.paren,
                String::from("Can only call functions and classes"),
            )),
        }
    }

    fn visit_grouping(&mut self, expr: Grouping) -> Self::Item {
        self.evaluate(expr.expression)
    }

    fn visit_literal(&mut self, expr: Literal) -> Self::Item {
        Ok(Rc::new(expr.value))
    }

    fn visit_variable(&mut self, expr: Variable) -> Self::Item {
        self.environment.borrow().get(expr.name)
    }

    fn visit_lambda(&mut self, expr: Lambda) -> Self::Item {
        let function = LambdaFunction {
            declaration: expr,
            closure: Rc::clone(&self.environment),
        };

        Ok(Rc::new(Object::Callable(Rc::new(function))))
    }
}

impl StmtVisitor for Interpreter {
    type Item = Result<(), Exception>;

    fn visit_variable_stmt(&mut self, stmt: VariableStmt) -> Self::Item {
        for var in stmt.variables {
            let value = if let Some(expr) = var.initializer {
                self.evaluate_and_map_error(expr)?
            } else {
                Rc::new(Object::Undefined)
            };

            self.environment.borrow_mut().define(var.name.lexeme, value);
        }

        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: FunctionStmt) -> Self::Item {
        let name = String::clone(&stmt.name.lexeme);
        let function = Function {
            declaration: stmt,
            closure: Rc::clone(&self.environment),
        };

        self.environment
            .borrow_mut()
            .define(name, Rc::new(Object::Callable(Rc::new(function))));

        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: ReturnStmt) -> Self::Item {
        let value = if let Some(value) = stmt.value {
            self.evaluate_and_map_error(value)?
        } else {
            Rc::new(Object::Undefined)
        };

        Err(Exception::ReturnException(value))
    }

    fn visit_while_stmt(&mut self, stmt: WhileStmt) -> Self::Item {
        while operations::is_truthy(&*self.evaluate_and_map_error(stmt.condition.clone())?) {
            let result = self.execute(stmt.body.clone());

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

    fn visit_if_stmt(&mut self, stmt: IfStmt) -> Self::Item {
        let condition = self.evaluate_and_map_error(stmt.condition)?;

        if operations::is_truthy(&condition) {
            self.execute(stmt.truth)?;
        } else if let Some(falsy_stmt) = stmt.falsy {
            self.execute(falsy_stmt)?;
        }

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: BlockStmt) -> Self::Item {
        self.execute_block(
            stmt.stmts,
            Environment::new(Some(Rc::clone(&self.environment))),
        )?;

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: PrintStmt) -> Self::Item {
        let value = self.evaluate_and_map_error(stmt.expression)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: ExpressionStmt) -> Self::Item {
        self.evaluate_and_map_error(stmt.expression)?;

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
