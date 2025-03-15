use super::{callable::Callable, Exception, Interpreter, RuntimeError};
use crate::{environment::Environment, object::Object, stmt::function_stmt::FunctionStmt};
use std::{cell::RefCell, rc::Rc};

pub struct Function {
    pub declaration: FunctionStmt,
    pub closure: Rc<RefCell<Environment>>,
}

impl Callable for Function {
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
            env.define(self.declaration.params[i].lexeme.to_string(), arg);
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
        format!("[Function: {}]", self.declaration.name.lexeme)
    }
}
