use crate::{errors::RuntimeError, Interpreter};

use super::Object;

pub trait Callable {
    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>;

    fn to_string(&self) -> String {
        "[Native Function]".to_string()
    }

    fn bind(&self, _: Object) -> Object {
        unreachable!()
    }
}
