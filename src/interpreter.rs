use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    errors::{RuntimeError, RuntimeException},
    expression::{
        Assignment, Binary, Call, Comma, Expression, ExpressionVisitor, Grouping, Lambda, Literal,
        Logical, Ternary, Unary, Variable,
    },
    globals, operations,
    stmt::{
        BlockStmt, ExpressionStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, Stmt, StmtVisitor,
        VariableStmt, WhileStmt,
    },
    Environment, Function, Lib, Object, Token, TokenType,
};

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<String, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));

        globals
            .borrow_mut()
            .define("clock", Rc::new(Object::Callable(Rc::new(globals::Clock))));

        Self {
            environment: Rc::clone(&globals),
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            if let Err(e) = self.execute(stmt) {
                match e {
                    RuntimeException::RuntimeError(runtime_error) => {
                        Lib::runtime_error(&runtime_error)
                    }
                    _ => unreachable!(),
                };
            }
        }
    }

    fn evaluate(&mut self, expr: &Expression) -> Result<Rc<Object>, RuntimeError> {
        expr.accept(self)
    }

    fn evaluate_and_map_error(
        &mut self,
        expr: &Expression,
    ) -> Result<Rc<Object>, RuntimeException> {
        self.evaluate(expr)
            .map_err(|e| RuntimeException::RuntimeError(e))
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeException> {
        stmt.accept(self)
    }

    pub fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        env: Environment,
    ) -> Result<(), RuntimeException> {
        let mut env_ref = Rc::new(RefCell::new(env));

        std::mem::swap(&mut self.environment, &mut env_ref);

        let result = stmts.into_iter().try_for_each(|stmt| self.execute(stmt));

        std::mem::swap(&mut self.environment, &mut env_ref);

        result
    }

    pub fn resolve(&mut self, hash: &str, depth: usize) {
        self.locals.insert(String::from(hash), depth);
    }

    fn look_up_variable(&mut self, name: &Token) -> Result<Rc<Object>, RuntimeError> {
        let distance = self.locals.get(name.identifier_hash.as_ref().unwrap());

        match distance {
            Some(depth) => self.environment.borrow().get_at(name, *depth),
            None => self.globals.borrow().get(&name),
        }
    }
}

impl ExpressionVisitor for Interpreter {
    type Item = Result<Rc<Object>, RuntimeError>;

    fn visit_comma(&mut self, expr: &Comma) -> Self::Item {
        self.evaluate(&expr.left)?;
        self.evaluate(&expr.right)
    }

    fn visit_lambda(&mut self, expr: &Lambda) -> Self::Item {
        let function = Function {
            declaration: expr.clone(),
            closure: Rc::clone(&self.environment),
        };

        Ok(Rc::new(Object::Callable(Rc::new(function))))
    }

    fn visit_assignment(&mut self, expr: &Assignment) -> Self::Item {
        let value = self.evaluate(&expr.expression)?;
        let distance = self.locals.get(expr.name.identifier_hash.as_ref().unwrap());

        match distance {
            Some(depth) => {
                self.environment
                    .borrow_mut()
                    .assign_at(&expr.name, Rc::clone(&value), *depth)?
            }
            None => self
                .globals
                .borrow_mut()
                .assign(&expr.name, Rc::clone(&value))?,
        };

        Ok(value)
    }

    fn visit_ternary(&mut self, expr: &Ternary) -> Self::Item {
        let condition = self.evaluate(&expr.condition)?;

        if operations::is_truthy(&condition) {
            self.evaluate(&expr.truth)
        } else {
            self.evaluate(&expr.falsy)
        }
    }

    fn visit_logical(&mut self, expr: &Logical) -> Self::Item {
        let left = self.evaluate(&expr.left)?;
        let is_truthy = operations::is_truthy(&left);
        let value = match expr.operator.token_type {
            TokenType::And => {
                if is_truthy {
                    self.evaluate(&expr.right)?
                } else {
                    left
                }
            }
            TokenType::Or => {
                if is_truthy {
                    left
                } else {
                    self.evaluate(&expr.right)?
                }
            }
            _ => unreachable!(),
        };

        Ok(value)
    }

    fn visit_binary(&mut self, expr: &Binary) -> Self::Item {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
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

    fn visit_unary(&mut self, expr: &Unary) -> Self::Item {
        let literal = self.evaluate(&expr.right)?;
        let literal = match expr.operator.token_type {
            TokenType::Bang => Object::Boolean(!operations::is_truthy(&literal)),
            TokenType::Minus => {
                let literal = match *literal {
                    Object::Number(number) => number,
                    Object::Boolean(boolean) => operations::bool_to_number(boolean),
                    _ => {
                        return Err(RuntimeError::new(
                            expr.operator.clone(),
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

    fn visit_call(&mut self, expr: &Call) -> Self::Item {
        let callee = self.evaluate(&expr.callee)?;
        let arguments = expr
            .arguments
            .iter()
            .map(|f| self.evaluate(f))
            .collect::<Result<Vec<_>, _>>()?;

        match &*callee {
            Object::Callable(c) => {
                let arity = c.arity();

                if arguments.len() < arity {
                    Err(RuntimeError::new(
                        expr.paren.clone(),
                        format!("Expected [{arity}] arguments got [{}]", arguments.len()),
                    ))
                } else {
                    c.call(self, arguments)
                }
            }
            _ => Err(RuntimeError::new(
                expr.paren.clone(),
                String::from("Can only call functions and classes"),
            )),
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Item {
        self.evaluate(&expr.expression)
    }

    fn visit_literal(&mut self, expr: &Literal) -> Self::Item {
        Ok(Rc::new(expr.value.clone()))
    }

    fn visit_variable(&mut self, expr: &Variable) -> Self::Item {
        return self.look_up_variable(&expr.name);
    }
}

impl StmtVisitor for Interpreter {
    type Item = Result<(), RuntimeException>;

    fn visit_empty_stmt(&mut self) -> Self::Item {
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Self::Item {
        self.evaluate_and_map_error(&stmt.value)?;

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Self::Item {
        let value = self.evaluate_and_map_error(&stmt.value)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_variable_stmt(&mut self, stmt: &VariableStmt) -> Self::Item {
        for var in &stmt.stmts {
            let value = if let Some(expr) = &var.initializer {
                self.evaluate_and_map_error(expr)?
            } else {
                Rc::new(Object::Undefined)
            };

            self.environment
                .borrow_mut()
                .define(&var.name.lexeme, value);
        }

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Self::Item {
        self.execute_block(
            &stmt.stmts,
            Environment::new(Some(Rc::clone(&self.environment))),
        )?;

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Self::Item {
        let condition = self.evaluate_and_map_error(&stmt.condition)?;

        if operations::is_truthy(&condition) {
            self.execute(&stmt.truth)?;
        } else if let Some(falsy_stmt) = &stmt.falsy {
            self.execute(falsy_stmt)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Item {
        while operations::is_truthy(&*self.evaluate_and_map_error(&stmt.condition)?) {
            let result = self.execute(&stmt.body);

            if let Err(e) = &result {
                match e {
                    RuntimeException::BreakException => break,
                    RuntimeException::ContinueException => continue,
                    _ => result?,
                }
            }
        }

        Ok(())
    }

    fn visit_break_stmt(&mut self, _: &Token) -> Self::Item {
        Err(RuntimeException::BreakException)
    }

    fn visit_continue_stmt(&mut self, _: &Token) -> Self::Item {
        Err(RuntimeException::ContinueException)
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Self::Item {
        let function = Function {
            declaration: stmt.clone(),
            closure: Rc::clone(&self.environment),
        };

        self.environment.borrow_mut().define(
            &stmt.name.lexeme,
            Rc::new(Object::Callable(Rc::new(function))),
        );

        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Self::Item {
        let value = if let Some(value) = &stmt.value {
            self.evaluate_and_map_error(value)?
        } else {
            Rc::new(Object::Undefined)
        };

        Err(RuntimeException::ReturnException(value))
    }
}
