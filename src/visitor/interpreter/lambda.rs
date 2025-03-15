use super::{callable::Callable, Exception, Interpreter, RuntimeError};
use crate::{environment::Environment, expression::lambda::Lambda, object::Object};
use std::{cell::RefCell, rc::Rc};

pub struct LambdaFunction {
    pub declaration: Lambda,
    pub closure: Rc<RefCell<Environment>>,
}

impl Callable for LambdaFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Rc<Object>>,
    ) -> Result<Rc<Object>, RuntimeError> {
        let mut env = Environment::new(Some(Rc::clone(&self.closure)));

        for (i, arg) in arguments.into_iter().enumerate() {
            env.define(String::clone(&self.declaration.params[i].lexeme), arg);
        }

        if let Err(err) = interpreter.execute_block(self.declaration.body.clone(), env) {
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
        String::from("[Function: (anonymous)]")
    }
}
