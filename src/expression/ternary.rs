use super::Expression;

#[derive(Clone)]
pub struct Ternary {
    pub condition: Expression,
    pub truth: Expression,
    pub falsy: Expression,
}
