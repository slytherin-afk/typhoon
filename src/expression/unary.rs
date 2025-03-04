use crate::scanner::token::Token;

use super::Expression;

pub struct Unary<'a> {
    pub operator: &'a Token<'a>,
    pub right: Expression<'a>,
}
