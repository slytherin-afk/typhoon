use crate::token::Token;

use super::Expr;

#[derive(Clone)]
pub struct Assignment {
    pub name: Token,
    pub value: Expr,
}
