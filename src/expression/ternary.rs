use super::Expression;

pub struct Ternary<'a> {
    pub condition: Expression<'a>,
    pub truth: Expression<'a>,
    pub falsy: Expression<'a>,
}
