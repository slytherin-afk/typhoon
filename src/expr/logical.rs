use crate::token::Token;

use super::Expr;

#[derive(Clone)]
pub struct Logical {
    pub operator: Token,
    pub left: Expr,
    pub right: Expr,
}
