use std::fmt;

use crate::callee::Callee;

#[derive(Clone, PartialEq)]
pub enum Object {
    Undefined,
    Number(f64),
    String(String),
    Boolean(bool),
    Callee(Callee),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Object::Undefined => "undefined",
            Object::Number(n) => &n.to_string(),
            Object::String(s) => &s.to_string(),
            Object::Boolean(b) => &b.to_string(),
            Object::Callee(callee) => {
                if let Some(to_string) = &(callee.to_string) {
                    &to_string()
                } else {
                    "[Function]"
                }
            }
        };

        write!(f, "{value}")
    }
}
