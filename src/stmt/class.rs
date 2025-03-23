use crate::token::Token;

use super::Stmt;

#[derive(Clone)]
pub struct Class {
    pub name: Token,
    pub methods: Vec<Stmt>,
    pub statics: Vec<Stmt>,
}
