use crate::token::Token;

#[derive(Clone)]
pub struct Super {
    pub keyword: Token,
    pub method: Token,
}
