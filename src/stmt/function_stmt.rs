use crate::{resolvable_function::ResolvableFunction, scanner::token::Token};

use super::Stmt;

#[derive(Clone)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl ResolvableFunction for FunctionStmt {
    fn params(&self) -> &Vec<Token> {
        &self.params
    }
    fn body(&self) -> &Vec<Stmt> {
        &self.body
    }

    fn name(&self) -> &str {
        &self.name.lexeme
    }
}
