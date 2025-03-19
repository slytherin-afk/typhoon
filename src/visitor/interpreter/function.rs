use super::{callable::Callable, environment::Environment, Exception, Interpreter, RuntimeError};
use crate::{object::Object, resolvable_function::ResolvableFunction};
use std::{cell::RefCell, rc::Rc};

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
                Exception::RuntimeError(runtime_error) => Err(runtime_error),
                Exception::ReturnException(object) => Ok(object),
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
