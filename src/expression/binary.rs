use crate::scanner::token::Token;

use super::Expression;

pub struct Binary {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}
