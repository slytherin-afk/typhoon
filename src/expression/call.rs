use crate::scanner::token::Token;

use super::Expression;

#[derive(Clone)]
pub struct Call {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
    pub paren: Token,
}
