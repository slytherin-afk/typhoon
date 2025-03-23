use crate::token::Token;

use super::Expr;

#[derive(Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}
