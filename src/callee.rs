use std::rc::Rc;

use crate::{
    object::Object,
    visitor::interpreter::{Interpreter, RuntimeError},
};

#[derive(Clone)]
pub struct Callee {
    pub arity: Rc<dyn Fn() -> usize>,
    pub call: Rc<dyn Fn(Interpreter, Vec<Object>) -> Result<Object, RuntimeError>>,
    pub to_string: Option<Rc<dyn Fn() -> String>>,
}

impl PartialEq for Callee {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.call, &other.call)
    }
}
