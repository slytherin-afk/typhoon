use crate::token::Token;

use super::Expr;

#[derive(Clone)]
pub struct Set {
    pub object: Expr,
    pub name: Token,
    pub value: Expr,
}
