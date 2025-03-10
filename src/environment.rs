use std::collections::HashMap;

use crate::{object::Object, scanner::token::Token, visitor::interpreter::RuntimeError};

pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: if let Some(env) = enclosing {
                Some(Box::new(env))
            } else {
                None
            },
        }
    }

    pub fn get(&self, token: &Token) -> Result<Object, RuntimeError> {
        if self.values.contains_key(&token.lexeme) {
            Ok(self.values.get(&token.lexeme).unwrap().clone())
        } else if let Some(env) = &self.enclosing {
            return env.get(token);
        } else {
            Err(RuntimeError {
                token: token.clone(),
                message: format!("Undefined variable '{}'", token.lexeme),
            })
        }
    }

    pub fn assign(&mut self, token: &Token, value: Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme.to_string(), value);

            Ok(())
        } else if let Some(env) = &mut self.enclosing {
            return env.assign(token, value);
        } else {
            Err(RuntimeError {
                token: token.clone(),
                message: format!("Undefined variable '{}'", token.lexeme),
            })
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }
}
