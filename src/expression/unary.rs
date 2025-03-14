use crate::scanner::token::Token;

use super::Expression;

#[derive(Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Expression,
}
