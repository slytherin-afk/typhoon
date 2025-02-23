use crate::scanner::token::Token;

use super::Expression;

pub struct Unary {
    pub operator: Token,
    pub right: Expression,
}
