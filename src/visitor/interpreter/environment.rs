use crate::{object::Object, scanner::token::Token, visitor::interpreter::RuntimeError};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    values: HashMap<String, Rc<Object>>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn get(&self, token: &Token) -> Result<Rc<Object>, RuntimeError> {
        if let Some(obj) = self.values.get(&token.lexeme) {
            Ok(Rc::clone(obj))
        } else if let Some(env) = &self.enclosing {
            env.borrow().get(token)
        } else {
            Err(RuntimeError::new(
                token.clone(),
                format!("Undefined variable '{}'", token.lexeme),
            ))
        }
    }

    pub fn get_at(&self, token: &Token, depth: usize) -> Result<Rc<Object>, RuntimeError> {
        if depth == 0 {
            Ok(Rc::clone(self.values.get(&token.lexeme).unwrap()))
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow()
                .get_at(token, depth - 1)
        }
    }

    pub fn assign(&mut self, token: &Token, value: Rc<Object>) -> Result<(), RuntimeError> {
        if self.values.contains_key(&token.lexeme) {
            self.values.insert(String::clone(&token.lexeme), value);

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

    pub fn assign_at(
        &mut self,
        token: &Token,
        value: Rc<Object>,
        depth: usize,
    ) -> Result<(), RuntimeError> {
        if depth == 0 {
            self.values.insert(String::clone(&token.lexeme), value);

            Ok(())
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign_at(token, value, depth - 1)
        }
    }

    pub fn define(&mut self, name: &str, value: Rc<Object>) -> &mut Self {
        self.values.insert(String::from(name), value);
        self
    }
}
