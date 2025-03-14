use std::rc::Rc;

use crate::{environment::Environment, object::Object, stmt::function_stmt::FunctionStmt};

use super::{callable::Callable, Exception, Interpreter, RuntimeError};

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
        arguments: Vec<Rc<Object>>,
    ) -> Result<Rc<Object>, RuntimeError> {
        let mut env = Environment::new(Some(Rc::clone(&interpreter.globals)));

        for (i, arg) in arguments.into_iter().enumerate() {
            env.define(self.declaration.params[i].lexeme.to_string(), arg);
        }

        interpreter
            .execute_block(self.declaration.body.clone(), env)
            .map_err(|e| {
                if let Exception::RuntimeError(e) = e {
                    e
                } else {
                    unreachable!()
                }
            })?;

        Ok(Rc::new(Object::Undefined))
    }

    fn to_string(&self) -> String {
        format!("[Function {}]", self.declaration.name.lexeme)
    }
}
