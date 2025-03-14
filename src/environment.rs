use crate::{object::Object, scanner::token::Token, visitor::interpreter::RuntimeError};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn get(&self, token: Token) -> Result<Object, RuntimeError> {
        if let Some(obj) = self.values.get(&token.lexeme) {
            Ok(obj.clone())
        } else if let Some(env) = &self.enclosing {
            env.borrow().get(token)
        } else {
            Err(RuntimeError::new(
                token.clone(),
                format!("Undefined variable '{}'", token.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, token: Token, value: Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme, value);

            Ok(())
        } else if let Some(env) = &mut self.enclosing {
            env.borrow_mut().assign(token, value)
        } else {
            Err(RuntimeError::new(
                token.clone(),
                format!("Undefined variable '{}'", token.lexeme),
            ))
        }
    }

    pub fn define(&mut self, name: String, value: Object) -> &mut Self {
        self.values.insert(name, value);
        self
    }
}
