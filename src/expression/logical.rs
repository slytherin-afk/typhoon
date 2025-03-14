use crate::scanner::token::Token;

use super::Expression;

#[derive(Clone)]
pub struct Logical {
    pub operator: Token,
    pub left: Expression,
    pub right: Expression,
}
