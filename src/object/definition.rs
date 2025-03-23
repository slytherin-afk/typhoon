use std::{fmt, rc::Rc};

use crate::{object::Callable, utils::bool_to_number};

use super::Object;

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Undefined => write!(f, "{}", "undefined"),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Callable(callee) => write!(f, "{}", callee.to_string()),
            Object::Instance(class_instance) => {
                write!(f, "{}", class_instance.to_string())
            }
            Object::CallableInstance(static_class) => {
                write!(f, "{}", Callable::to_string(static_class.as_ref()))
            }
        }
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
            (Object::Instance(a), Object::Instance(b)) => Rc::ptr_eq(a, b),
            (Object::CallableInstance(a), Object::CallableInstance(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
