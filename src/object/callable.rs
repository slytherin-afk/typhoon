use crate::{errors::RuntimeError, interpreter::Interpreter};

use super::Object;

pub trait Callable {
    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>;

    fn to_string(&self) -> String;

    fn bind(&self, _: Object) -> Object;
}
