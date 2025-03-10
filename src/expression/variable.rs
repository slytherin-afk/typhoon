use crate::scanner::token::Token;

pub struct Variable<'a> {
    pub name: &'a Token,
}
