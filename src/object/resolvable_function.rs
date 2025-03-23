use crate::{
    expr,
    stmt::{self, Stmt},
    token::Token,
};

pub trait ResolvableFunction: 'static {
    fn params(&self) -> &Vec<Token>;

    fn body(&self) -> &Vec<Stmt>;

    fn name(&self) -> &str;
}

impl ResolvableFunction for stmt::Function {
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

impl ResolvableFunction for expr::Lambda {
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
