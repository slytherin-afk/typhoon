use crate::scanner::token_type::TokenType;

use super::{
    binary::Binary,
    comma::Comma,
    grouping::Grouping,
    literal::{Literal, LiteralValue},
    ternary::Ternary,
    unary::Unary,
    Expression, ExpressionVisitor,
};

pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(expr: &mut Expression) -> LiteralValue {
        expr.accept(&Self)
    }

    fn handle_addition(left: LiteralValue, right: LiteralValue) -> LiteralValue {
        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Number(l + r),
            (LiteralValue::String(l), LiteralValue::String(r)) => LiteralValue::String(l + &r),
            (LiteralValue::Number(l), LiteralValue::Boolean(b)) => {
                LiteralValue::Number(l + (b as i64 as f64))
            }
            (LiteralValue::Boolean(b), LiteralValue::Number(r)) => {
                LiteralValue::Number((b as i64 as f64) + r)
            }
            _ => todo!(),
        }
    }

    fn handle_subtraction(left: LiteralValue, right: LiteralValue) -> LiteralValue {
        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Number(l - r),
            (LiteralValue::Number(l), LiteralValue::Boolean(b)) => {
                LiteralValue::Number(l - (b as i64 as f64))
            }
            (LiteralValue::Boolean(b), LiteralValue::Number(r)) => {
                LiteralValue::Number((b as i64 as f64) - r)
            }
            _ => todo!(),
        }
    }

    fn handle_multiplication(left: LiteralValue, right: LiteralValue) -> LiteralValue {
        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Number(l * r),
            (LiteralValue::Number(l), LiteralValue::Boolean(b)) => {
                LiteralValue::Number(l * (b as i64 as f64))
            }
            (LiteralValue::Boolean(b), LiteralValue::Number(r)) => {
                LiteralValue::Number((b as i64 as f64) * r)
            }
            _ => todo!(),
        }
    }

    fn handle_division(left: LiteralValue, right: LiteralValue) -> LiteralValue {
        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                if r == 0.0 {
                    todo!()
                } else {
                    LiteralValue::Number(l / r)
                }
            }
            (LiteralValue::Number(l), LiteralValue::Boolean(b)) => {
                if b {
                    LiteralValue::Number(l)
                } else {
                    todo!()
                }
            }
            (LiteralValue::Boolean(b), LiteralValue::Number(r)) => {
                if r == 0.0 {
                    todo!()
                } else {
                    LiteralValue::Number((b as i64 as f64) / r)
                }
            }
            _ => todo!(),
        }
    }

    fn handle_comparison<F>(left: LiteralValue, right: LiteralValue, comparator: F) -> LiteralValue
    where
        F: Fn(f64, f64) -> bool,
    {
        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                LiteralValue::Boolean(comparator(l, r))
            }
            (LiteralValue::Number(l), LiteralValue::Boolean(b)) => {
                LiteralValue::Boolean(comparator(l, b as i64 as f64))
            }
            (LiteralValue::Boolean(b), LiteralValue::Number(r)) => {
                LiteralValue::Boolean(comparator(b as i64 as f64, r))
            }
            _ => todo!(),
        }
    }
}

impl ExpressionVisitor for Interpreter {
    type Item = LiteralValue;

    fn visit_comma(&self, expr: &mut Comma) -> Self::Item {
        expr.left.accept(&Self);
        expr.right.accept(&Self)
    }

    fn visit_ternary(&self, expr: &mut Ternary) -> Self::Item {
        let condition = expr.condition.accept(&Self);

        match condition {
            LiteralValue::Boolean(true) => expr.truth.accept(&Self),
            LiteralValue::Boolean(false) => expr.falsy.accept(&Self),
            LiteralValue::Number(n) => {
                if n != 0.0 {
                    expr.truth.accept(&Self)
                } else {
                    expr.falsy.accept(&Self)
                }
            }
            LiteralValue::String(s) => {
                if !s.is_empty() {
                    expr.truth.accept(&Self)
                } else {
                    expr.falsy.accept(&Self)
                }
            }
            LiteralValue::None => expr.falsy.accept(&Self),
        }
    }

    fn visit_binary(&self, expr: &mut Binary) -> Self::Item {
        let left = expr.left.accept(&Self);
        let right = expr.right.accept(&Self);

        match expr.operator.token_type {
            TokenType::Plus => Self::handle_addition(left, right),
            TokenType::Minus => Self::handle_subtraction(left, right),
            TokenType::Star => Self::handle_multiplication(left, right),
            TokenType::Slash => Self::handle_division(left, right),
            TokenType::Greater => Self::handle_comparison(left, right, |l, r| l > r),
            TokenType::GreaterEqual => Self::handle_comparison(left, right, |l, r| l >= r),
            TokenType::Less => Self::handle_comparison(left, right, |l, r| l < r),
            TokenType::LessEqual => Self::handle_comparison(left, right, |l, r| l <= r),
            TokenType::EqualEqual => LiteralValue::Boolean(left == right),
            TokenType::BangEqual => LiteralValue::Boolean(left != right),
            _ => todo!(),
        }
    }

    fn visit_unary(&self, expr: &mut Unary) -> Self::Item {
        let literal = expr.right.accept(&Self);

        match expr.operator.token_type {
            TokenType::Bang => match literal {
                LiteralValue::None => LiteralValue::Boolean(true),
                LiteralValue::Boolean(boolean) => LiteralValue::Boolean(!boolean),
                LiteralValue::Number(number) => {
                    if number == 0.0 {
                        LiteralValue::Boolean(true)
                    } else {
                        LiteralValue::Boolean(false)
                    }
                }
                LiteralValue::String(string) => {
                    if string.is_empty() {
                        LiteralValue::Boolean(true)
                    } else {
                        LiteralValue::Boolean(false)
                    }
                }
            },

            TokenType::Minus => match literal {
                LiteralValue::Number(number) => LiteralValue::Number(-number),
                LiteralValue::Boolean(boolean) => LiteralValue::Number(-(boolean as i64) as f64),
                _ => todo!(),
            },

            _ => unreachable!(),
        }
    }

    fn visit_grouping(&self, expr: &mut Grouping) -> Self::Item {
        expr.expression.accept(&Self)
    }

    fn visit_literal(&self, expr: &mut Literal) -> Self::Item {
        expr.value.clone()
    }
}
