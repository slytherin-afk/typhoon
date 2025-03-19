use crate::visitor::interpreter::{callable::Callable, operations::bool_to_number};
use std::{fmt, rc::Rc};

#[derive(Clone)]
pub enum Object {
    Undefined,
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Rc<dyn Callable>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Object::Undefined => "undefined",
            Object::Number(n) => &n.to_string(),
            Object::String(s) => &s.to_string(),
            Object::Boolean(b) => &b.to_string(),
            Object::Callable(callee) => &callee.to_string(),
        };

        write!(f, "{value}")
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Undefined, Object::Undefined) => true,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::Number(a), Object::Boolean(b)) => *a == bool_to_number(*b),
            (Object::Boolean(a), Object::Number(b)) => bool_to_number(*a) == *b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Callable(a), Object::Callable(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
