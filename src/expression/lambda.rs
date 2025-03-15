use crate::{scanner::token::Token, stmt::Stmt};

#[derive(Clone)]
pub struct Lambda {
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}
