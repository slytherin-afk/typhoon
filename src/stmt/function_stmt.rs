use crate::scanner::token::Token;

use super::Stmt;

#[derive(Clone)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}
