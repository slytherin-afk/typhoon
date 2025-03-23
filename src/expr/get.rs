use crate::token::Token;

use super::Expr;

#[derive(Clone)]
pub struct Get {
    pub object: Expr,
    pub name: Token,
}
