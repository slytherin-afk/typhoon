use std::rc::Rc;

use crate::{
    object::Object,
    visitor::interpreter::{Interpreter, RuntimeError},
};

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
