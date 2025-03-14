use crate::scanner::token::Token;

use super::Expression;

#[derive(Clone)]
pub struct Assignment {
    pub name: Token,
    pub expression: Expression,
}
