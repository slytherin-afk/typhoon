mod globals;
mod operations;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    environment::Environment,
    errors::{RuntimeError, VMException},
    expr::{self, Expr, ExprVisitor},
    object::{Callable, Class, Function, Instance, Object},
    stmt::{self, Stmt, StmtVisitor},
    token::Token,
    token_type::TokenType,
    utils::{bool_to_number, is_truthy},
    Lib,
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
            .define("clock", Object::Callable(Rc::new(globals::Clock)));

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
                    VMException::RuntimeError(runtime_error) => Lib::runtime_error(&runtime_error),
                    _ => unreachable!(),
                };
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        expr.accept(self)
    }

    fn evaluate_and_map_error(&mut self, expr: &Expr) -> Result<Object, VMException> {
        self.evaluate(expr)
            .map_err(|e| VMException::RuntimeError(e))
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), VMException> {
        stmt.accept(self)
    }

    pub fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        env: Environment,
    ) -> Result<(), VMException> {
        let mut env_ref = Rc::new(RefCell::new(env));

        std::mem::swap(&mut self.environment, &mut env_ref);

        let result = stmts.into_iter().try_for_each(|stmt| self.execute(stmt));

        std::mem::swap(&mut self.environment, &mut env_ref);

        result
    }

    pub fn resolve(&mut self, hash: &str, depth: usize) {
        self.locals.insert(String::from(hash), depth);
    }

    fn look_up_variable(&mut self, name: &Token) -> Result<Object, RuntimeError> {
        let distance = self.locals.get(name.identifier_hash.as_ref().unwrap());

        match distance {
            Some(depth) => self.environment.borrow().get_at(*depth, &name.lexeme),
            None => self.globals.borrow().get(&name),
        }
    }
}

impl ExprVisitor for Interpreter {
    type Item = Result<Object, RuntimeError>;

    fn visit_comma(&mut self, expr: &expr::Comma) -> Self::Item {
        self.evaluate(&expr.left)?;
        self.evaluate(&expr.right)
    }

    fn visit_lambda(&mut self, expr: &expr::Lambda) -> Self::Item {
        let function = Function::new(Rc::new(expr.clone()), Rc::clone(&self.environment), false);

        Ok(Object::Callable(Rc::new(function)))
    }

    fn visit_assignment(&mut self, expr: &expr::Assignment) -> Self::Item {
        let value = self.evaluate(&expr.value)?;
        let distance = self.locals.get(expr.name.identifier_hash.as_ref().unwrap());

        match distance {
            Some(depth) => {
                self.environment
                    .borrow_mut()
                    .assign_at(*depth, &expr.name.lexeme, value.clone())?
            }
            None => self
                .globals
                .borrow_mut()
                .assign(&expr.name, value.clone())?,
        };

        Ok(value)
    }

    fn visit_set(&mut self, expr: &expr::Set) -> Self::Item {
        let object = self.evaluate(&expr.object)?;

        fn set_field<T: Instance + ?Sized>(
            instance: Rc<T>,
            expr: &expr::Set,
            interpreter: &mut Interpreter,
        ) -> Result<Object, RuntimeError> {
            let value = interpreter.evaluate(&expr.value)?;

            instance.set(&expr.name, value.clone())?;

            Ok(value)
        }

        match object {
            Object::Instance(class_instance) => set_field(class_instance, expr, self),
            Object::CallableInstance(class_instance) => set_field(class_instance, expr, self),
            _ => Err(RuntimeError {
                token: expr.name.clone(),
                message: "Only class instances have fields".to_string(),
            }),
        }
    }

    fn visit_ternary(&mut self, expr: &expr::Ternary) -> Self::Item {
        let condition = self.evaluate(&expr.condition)?;

        if is_truthy(&condition) {
            self.evaluate(&expr.truth)
        } else {
            self.evaluate(&expr.falsy)
        }
    }

    fn visit_logical(&mut self, expr: &expr::Logical) -> Self::Item {
        let left = self.evaluate(&expr.left)?;
        let is_truthy = is_truthy(&left);
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

    fn visit_binary(&mut self, expr: &expr::Binary) -> Self::Item {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Plus => operations::handle_addition(&left, &right, &expr.operator),
            TokenType::Minus => operations::handle_subtraction(&left, &right, &expr.operator),
            TokenType::Star => operations::handle_multiplication(&left, &right, &expr.operator),
            TokenType::Slash => operations::handle_division(&left, &right, &expr.operator),
            TokenType::Percentage => operations::handle_modulus(&left, &right, &expr.operator),
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
        }
    }

    fn visit_unary(&mut self, expr: &expr::Unary) -> Self::Item {
        let literal = self.evaluate(&expr.right)?;
        let literal = match expr.operator.token_type {
            TokenType::Bang => Object::Boolean(!is_truthy(&literal)),
            TokenType::Minus => {
                let literal = match literal {
                    Object::Number(number) => number,
                    Object::Boolean(boolean) => bool_to_number(boolean),
                    _ => {
                        return Err(RuntimeError {
                            token: expr.operator.clone(),
                            message: String::from("Unary minus requires number or boolean operand"),
                        })
                    }
                };

                Object::Number(-literal)
            }
            _ => unreachable!(),
        };

        Ok(literal)
    }

    fn visit_call(&mut self, expr: &expr::Call) -> Self::Item {
        let callee = self.evaluate(&expr.callee)?;
        let arguments = expr
            .arguments
            .iter()
            .map(|f| self.evaluate(f))
            .collect::<Result<Vec<_>, _>>()?;

        fn check_and_call<T: Callable + ?Sized>(
            callable: Rc<T>,
            expr: &expr::Call,
            interpreter: &mut Interpreter,
            arguments: Vec<Object>,
        ) -> Result<Object, RuntimeError> {
            let arity = callable.arity();

            if arguments.len() < arity {
                Err(RuntimeError {
                    token: expr.paren.clone(),
                    message: format!("Expected [{arity}] arguments got [{}]", arguments.len()),
                })
            } else {
                callable.call(interpreter, arguments)
            }
        }

        match callee {
            Object::Callable(c) => check_and_call(c, expr, self, arguments),
            Object::CallableInstance(c) => check_and_call(c, expr, self, arguments),
            _ => Err(RuntimeError {
                token: expr.paren.clone(),
                message: "Can only call functions and classes".to_string(),
            }),
        }
    }

    fn visit_get(&mut self, expr: &expr::Get) -> Self::Item {
        let object = self.evaluate(&expr.object)?;

        match &object {
            Object::Instance(class_instance) => class_instance.get(object.clone(), &expr.name),
            Object::CallableInstance(class_instance) => {
                class_instance.get(object.clone(), &expr.name)
            }
            _ => Err(RuntimeError {
                token: expr.name.clone(),
                message: String::from("Only class instance have known properties"),
            }),
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Self::Item {
        self.evaluate(expr)
    }

    fn visit_variable(&mut self, expr: &Token) -> Self::Item {
        self.look_up_variable(expr)
    }

    fn visit_this(&mut self, expr: &Token) -> Self::Item {
        self.look_up_variable(expr)
    }

    fn visit_super(&mut self, expr: &expr::Super) -> Self::Item {
        let distance = self
            .locals
            .get(expr.keyword.identifier_hash.as_ref().unwrap())
            .unwrap();
        let super_class = self.environment.borrow().get_at(*distance, "super")?;
        let object = self.environment.borrow().get_at(distance - 1, "this")?;

        if let Object::CallableInstance(super_class) = super_class {
            if let Some(class) = super_class.as_any().downcast_ref::<Class>() {
                match class.find_method(&expr.method.lexeme) {
                    Some(method) => {
                        if let Object::Callable(method) = method {
                            return Ok(method.bind(object));
                        }
                    }
                    None => Err(RuntimeError {
                        token: expr.method.clone(),
                        message: format!("Undefined property '{}'", expr.method.lexeme),
                    })?,
                }
            }
        };

        unreachable!();
    }

    fn visit_literal(&mut self, expr: &Object) -> Self::Item {
        Ok(expr.clone())
    }
}

impl StmtVisitor for Interpreter {
    type Item = Result<(), VMException>;

    fn visit_empty_stmt(&mut self) -> Self::Item {
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &Expr) -> Self::Item {
        self.evaluate_and_map_error(stmt)?;

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Expr) -> Self::Item {
        let value = self.evaluate_and_map_error(stmt)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_variable_stmt(&mut self, stmt: &Vec<stmt::VariableDeclaration>) -> Self::Item {
        for var in stmt {
            let value = if let Some(expr) = &var.initializer {
                self.evaluate_and_map_error(expr)?
            } else {
                Object::Undefined
            };

            self.environment
                .borrow_mut()
                .define(&var.name.lexeme, value);
        }

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Vec<Stmt>) -> Self::Item {
        self.execute_block(stmt, Environment::new(Some(Rc::clone(&self.environment))))?;

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Self::Item {
        let condition = self.evaluate_and_map_error(&stmt.condition)?;

        if is_truthy(&condition) {
            self.execute(&stmt.truth)?;
        } else if let Some(falsy_stmt) = &stmt.falsy {
            self.execute(falsy_stmt)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Self::Item {
        while is_truthy(&self.evaluate_and_map_error(&stmt.condition)?) {
            let result = self.execute(&stmt.body);

            if let Err(e) = &result {
                match e {
                    VMException::BreakException => break,
                    VMException::ContinueException => continue,
                    _ => result?,
                }
            }
        }

        Ok(())
    }

    fn visit_break_stmt(&mut self, _: &Token) -> Self::Item {
        Err(VMException::BreakException)
    }

    fn visit_continue_stmt(&mut self, _: &Token) -> Self::Item {
        Err(VMException::ContinueException)
    }

    fn visit_function_stmt(&mut self, stmt: &stmt::Function) -> Self::Item {
        let function = Function::new(Rc::new(stmt.clone()), Rc::clone(&self.environment), false);

        self.environment
            .borrow_mut()
            .define(&stmt.name.lexeme, Object::Callable(Rc::new(function)));

        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Self::Item {
        let value = if let Some(value) = &stmt.value {
            self.evaluate_and_map_error(value)?
        } else {
            Object::Undefined
        };

        Err(VMException::ReturnException(value))
    }

    fn visit_class_stmt(&mut self, stmt: &stmt::Class) -> Self::Item {
        let super_class = if let Some(Expr::Variable(super_class)) = &stmt.super_class {
            let super_class_object =
                self.evaluate_and_map_error(stmt.super_class.as_ref().unwrap())?;

            match super_class_object {
                Object::CallableInstance(callable_instance) => Some(callable_instance),
                _ => Err(VMException::RuntimeError(RuntimeError {
                    token: *super_class.clone(),
                    message: String::from("Superclass must be a class"),
                }))?,
            }
        } else {
            None
        };

        self.environment
            .borrow_mut()
            .define(&stmt.name.lexeme, Object::Undefined);

        if let Some(super_class) = &super_class {
            self.environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                &self.environment,
            )))));

            self.environment
                .borrow_mut()
                .define("super", Object::CallableInstance(Rc::clone(super_class)));
        }

        let mut statics = HashMap::new();

        for method in &stmt.statics {
            if let Stmt::Function(function_stmt) = method {
                let function = Function::new(
                    Rc::new(*function_stmt.clone()),
                    Rc::clone(&self.environment),
                    false,
                );

                statics.insert(
                    String::clone(&function_stmt.name.lexeme),
                    Object::Callable(Rc::new(function)),
                );
            }
        }

        let mut methods = HashMap::new();

        for method in &stmt.methods {
            if let Stmt::Function(function_stmt) = method {
                let function = Function::new(
                    Rc::new(*function_stmt.clone()),
                    Rc::clone(&self.environment),
                    function_stmt.name.lexeme.eq("init"),
                );

                methods.insert(
                    String::clone(&function_stmt.name.lexeme),
                    Object::Callable(Rc::new(function)),
                );
            }
        }

        let class = Class::new(&stmt.name.lexeme, super_class, statics, methods);

        if let Some(_) = &stmt.super_class {
            let previous = Rc::clone(self.environment.borrow().enclosing.as_ref().unwrap());
            self.environment = previous;
        }

        self.environment
            .borrow_mut()
            .assign(&stmt.name, Object::CallableInstance(Rc::new(class)))
            .unwrap();

        Ok(())
    }
}
