use crate::scanner::token::Token;

use super::Expression;

pub struct Assignment<'a> {
    pub name: &'a Token,
    pub expression: Expression<'a>,
}
