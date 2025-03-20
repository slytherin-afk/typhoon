use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::{RuntimeError, RuntimeException},
    stmt::Stmt,
    Environment, Interpreter, Object, Token,
};

pub trait ResolvableFunction {
    fn params(&self) -> &Vec<Token>;
    fn body(&self) -> &Vec<Stmt>;
    fn name(&self) -> &str;
}

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Rc<Object>>,
    ) -> Result<Rc<Object>, RuntimeError>;
    fn to_string(&self) -> String {
        "[Native Function]".to_string()
    }
}

pub struct Function<T: ResolvableFunction> {
    pub declaration: T,
    pub closure: Rc<RefCell<Environment>>,
}

impl<T: ResolvableFunction> Callable for Function<T> {
    fn arity(&self) -> usize {
        self.declaration.params().len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Rc<Object>>,
    ) -> Result<Rc<Object>, RuntimeError> {
        let mut env = Environment::new(Some(Rc::clone(&self.closure)));

        for (param, arg) in self.declaration.params().iter().zip(arguments) {
            env.define(&param.lexeme, arg);
        }

        if let Err(err) = interpreter.execute_block(&self.declaration.body(), env) {
            match err {
                RuntimeException::RuntimeError(runtime_error) => Err(runtime_error),
                RuntimeException::ReturnException(object) => Ok(object),
                _ => unreachable!(),
            }
        } else {
            Ok(Rc::new(Object::Undefined))
        }
    }

    fn to_string(&self) -> String {
        format!("[Function: ({})]", self.declaration.name())
    }
}
