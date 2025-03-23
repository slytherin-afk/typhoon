use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    errors::{RuntimeError, VMException},
    interpreter::Interpreter,
};

use super::{Callable, Object, ResolvableFunction};

pub struct Function<T: ResolvableFunction> {
    declaration: Rc<T>,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl<T: ResolvableFunction> Function<T> {
    pub fn new(
        declaration: Rc<T>,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Self {
        Self {
            declaration,
            closure,
            is_initializer,
        }
    }
}

impl<T: ResolvableFunction> Callable for Function<T> {
    fn arity(&self) -> usize {
        self.declaration.params().len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut env = Environment::new(Some(Rc::clone(&self.closure)));

        for (param, arg) in self.declaration.params().iter().zip(arguments) {
            env.define(&param.lexeme, arg);
        }

        if let Err(err) = interpreter.execute_block(&self.declaration.body(), env) {
            return match err {
                VMException::RuntimeError(runtime_error) => Err(runtime_error),
                VMException::ReturnException(object) => {
                    if self.is_initializer {
                        return self.closure.borrow().get_at(0, "this");
                    }

                    Ok(object)
                }
                _ => unreachable!(),
            };
        }

        Ok(Object::Undefined)
    }

    fn to_string(&self) -> String {
        format!("[Function: ({})]", self.declaration.name())
    }

    fn bind(&self, instance: Object) -> Object {
        let mut env = Environment::new(Some(Rc::clone(&self.closure)));

        env.define("this", instance);

        Object::Callable(Rc::new(Function::new(
            Rc::clone(&self.declaration),
            Rc::new(RefCell::new(env)),
            self.is_initializer,
        )))
    }
}
