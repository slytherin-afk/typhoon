use crate::scanner::token::Token;

use super::Expression;

pub struct Call<'a> {
    pub callee: Expression<'a>,
    pub arguments: Vec<Expression<'a>>,
    pub paren: &'a Token,
}
