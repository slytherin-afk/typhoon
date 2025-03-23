use crate::token::Token;

use super::Expr;

#[derive(Clone)]
pub struct Call {
    pub callee: Expr,
    pub arguments: Vec<Expr>,
    pub paren: Token,
}
