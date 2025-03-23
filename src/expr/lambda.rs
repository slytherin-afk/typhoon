use crate::{stmt::Stmt, token::Token};

#[derive(Clone)]
pub struct Lambda {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}
