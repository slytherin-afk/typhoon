use super::Expr;

#[derive(Clone)]
pub struct Comma {
    pub left: Expr,
    pub right: Expr,
}
