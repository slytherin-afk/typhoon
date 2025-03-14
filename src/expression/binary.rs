use crate::scanner::token::Token;

use super::Expression;

#[derive(Clone)]
pub struct Binary {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}
