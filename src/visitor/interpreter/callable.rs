use crate::{
    object::Object,
    visitor::interpreter::{Interpreter, RuntimeError},
};

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
}
