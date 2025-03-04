use crate::scanner::token::Token;

use super::Expression;

pub struct Binary<'a> {
    pub left: Expression<'a>,
    pub operator: &'a Token<'a>,
    pub right: Expression<'a>,
}
