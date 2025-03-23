use crate::token::Token;

use super::Expr;

#[derive(Clone)]
pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}
