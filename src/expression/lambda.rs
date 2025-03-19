use crate::{resolvable_function::ResolvableFunction, scanner::token::Token, stmt::Stmt};

#[derive(Clone)]
pub struct Lambda {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl ResolvableFunction for Lambda {
    fn params(&self) -> &Vec<Token> {
        &self.params
    }
    fn body(&self) -> &Vec<Stmt> {
        &self.body
    }

    fn name(&self) -> &str {
        "anonymous"
    }
}
