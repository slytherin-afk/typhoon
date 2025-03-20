use crate::{Object, Token};

#[derive(Debug)]
pub struct ParseError;

pub enum RuntimeException {
    RuntimeError(RuntimeError),
    ReturnException(Object),
    BreakException,
    ContinueException,
}

pub struct BreakException;

pub struct ContinueException;

pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
