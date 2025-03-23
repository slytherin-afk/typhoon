use crate::{object::Object, token::Token};

#[derive(Debug)]
pub struct SyntaxError;

pub enum VMException {
    RuntimeError(RuntimeError),
    ReturnException(Object),
    BreakException,
    ContinueException,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

pub struct BreakException;

pub struct ContinueException;
