use super::{callable::Callable, Exception, Interpreter, RuntimeError};
use crate::{environment::Environment, object::Object, stmt::function_stmt::FunctionStmt};
use std::rc::Rc;

pub struct Function {
    pub declaration: FunctionStmt,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut env = Environment::new(Some(Rc::clone(&interpreter.globals)));

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
            Ok(Object::Undefined)
        }
    }

    fn to_string(&self) -> String {
        format!("[Function: {}]", self.declaration.name.lexeme)
    }
}
