use crate::scanner::token::Token;

use super::Expression;

pub struct Logical<'a> {
    pub operator: &'a Token,
    pub left: Expression<'a>,
    pub right: Expression<'a>,
}
