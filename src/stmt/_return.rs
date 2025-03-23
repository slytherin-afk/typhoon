use crate::{expr::Expr, token::Token};

#[derive(Clone)]
pub struct Return {
    pub keyword: Token,
    pub value: Option<Expr>,
}
