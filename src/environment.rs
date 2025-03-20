use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{errors::RuntimeError, Object, Token};

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

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        if let Some(obj) = self.values.get(&name.lexeme) {
            Ok(obj.clone())
        } else if let Some(env) = &self.enclosing {
            env.borrow().get(name)
        } else {
            Err(RuntimeError::new(
                name.clone(),
                format!("Undefined variable '{}'", name.lexeme),
            ))
        }
    }

    pub fn get_at(&self, depth: usize, name: &str) -> Result<Object, RuntimeError> {
        if depth == 0 {
            Ok(self.values.get(name).unwrap().clone())
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow()
                .get_at(depth - 1, name)
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(String::clone(&name.lexeme), value);

            Ok(())
        } else if let Some(env) = &mut self.enclosing {
            env.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::new(
                name.clone(),
                format!("Undefined variable '{}'", name.lexeme),
            ))
        }
    }

    pub fn assign_at(
        &mut self,
        depth: usize,
        name: &str,
        value: Object,
    ) -> Result<(), RuntimeError> {
        if depth == 0 {
            self.values.insert(String::from(name), value);

            Ok(())
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign_at(depth - 1, name, value)
        }
    }

    pub fn define(&mut self, name: &str, value: Object) -> &mut Self {
        self.values.insert(String::from(name), value);
        self
    }
}
