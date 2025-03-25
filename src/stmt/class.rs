use crate::{expr::Expr, token::Token};

use super::Stmt;

#[derive(Clone)]
pub struct Class {
    pub name: Token,
    pub super_class: Option<Expr>,
    pub methods: Vec<Stmt>,
    pub statics: Vec<Stmt>,
}
