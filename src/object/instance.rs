use std::{cell::RefCell, collections::HashMap};

use crate::{errors::RuntimeError, Token};

use super::{class::Class, Object};

pub trait Instance {
    fn get(&self, this: Object, name: &Token) -> Result<Object, RuntimeError>;
    fn set(&self, name: &Token, value: Object) -> Result<(), RuntimeError>;
    fn to_string(&self) -> String;
}
pub struct ClassInstance {
    class: Class,
    fields: RefCell<HashMap<String, Object>>,
}

impl ClassInstance {
    pub fn new(class: Class) -> Self {
        Self {
            class,
            fields: RefCell::new(HashMap::new()),
        }
    }
}

impl Instance for ClassInstance {
    fn get(&self, this: Object, name: &Token) -> Result<Object, RuntimeError> {
        if let Some(field) = self.fields.borrow().get(&name.lexeme) {
            return Ok(field.clone());
        }

        if let Some(method) = self.class.find_method(&name.lexeme) {
            return match &method {
                Object::Callable(callable) => Ok(callable.bind(this)),
                _ => unreachable!(),
            };
        }

        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined property '{}'", name.lexeme),
        ))
    }

    fn set(&self, name: &Token, value: Object) -> Result<(), RuntimeError> {
        self.fields
            .borrow_mut()
            .insert(String::clone(&name.lexeme), value);

        Ok(())
    }

    fn to_string(&self) -> String {
        format!("[Class Instance: ({})]", self.class.internal.name)
    }
}
