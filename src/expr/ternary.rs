use super::Expr;

#[derive(Clone)]
pub struct Ternary {
    pub condition: Expr,
    pub truth: Expr,
    pub falsy: Expr,
}
