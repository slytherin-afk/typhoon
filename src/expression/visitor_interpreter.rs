use crate::{
    scanner::{token::Token, token_type::TokenType},
    Typhoon,
};

use super::{
    binary::Binary,
    comma::Comma,
    grouping::Grouping,
    literal::{Literal, LiteralValue},
    ternary::Ternary,
    unary::Unary,
    Expression, ExpressionVisitor,
};

pub struct RuntimeError {
    pub token: Token,
    pub message: &'static str,
}

impl RuntimeError {
    pub fn new(token: Token, message: &'static str) -> Self {
        Self { token, message }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(expr: &mut Expression, typhoon: &mut Typhoon) {
        match Self::evaluate(expr) {
            Ok(value) => println!("{value}"),
            Err(error) => typhoon.runtime_error(&error),
        }
    }

    fn evaluate(expr: &mut Expression) -> Result<LiteralValue, RuntimeError> {
        expr.accept(&Self)
    }

    fn handle_addition(
        left: &LiteralValue,
        right: &LiteralValue,
        operator: &Token,
    ) -> Result<LiteralValue, RuntimeError> {
        Self::validate_addition_operands(&left, &right, &operator)?;

        match (left, right) {
            (LiteralValue::String(l), LiteralValue::String(r)) => {
                return Ok(LiteralValue::String(format!("{l}{r}")))
            }
            _ => {
                let result = Self::to_number(left) + Self::to_number(right);

                return Ok(LiteralValue::Number(result));
            }
        };
    }

    fn handle_arithmetic(
        left: &LiteralValue,
        right: &LiteralValue,
        operator: &Token,
    ) -> Result<LiteralValue, RuntimeError> {
        Self::validate_arithmetic_operands(left, right, operator)?;

        let left_number = Self::to_number(left);
        let right_number = Self::to_number(right);
        let value = match operator.token_type {
            TokenType::Minus => left_number - right_number,
            TokenType::Star => left_number * right_number,
            TokenType::Slash => {
                if right_number == 0.0 {
                    return Err(RuntimeError::new(operator.clone(), "Not divisible by Zero"));
                } else {
                    left_number / right_number
                }
            }
            _ => unreachable!(),
        };

        Ok(LiteralValue::Number(value))
    }

    fn handle_comparison(
        left: &LiteralValue,
        right: &LiteralValue,
        operator: &Token,
    ) -> Result<LiteralValue, RuntimeError> {
        Self::validate_addition_operands(&left, &right, &operator)?;

        let value = match (left, right) {
            (LiteralValue::String(l), LiteralValue::String(r)) => match operator.token_type {
                TokenType::Greater => l > r,
                TokenType::GreaterEqual => l >= r,
                TokenType::Less => l < r,
                TokenType::LessEqual => l <= r,
                _ => unreachable!(),
            },
            _ => {
                let l = Self::to_number(left);
                let r = Self::to_number(right);

                match operator.token_type {
                    TokenType::Greater => l > r,
                    TokenType::GreaterEqual => l >= r,
                    TokenType::Less => l < r,
                    TokenType::LessEqual => l <= r,
                    _ => unreachable!(),
                }
            }
        };

        Ok(LiteralValue::Boolean(value))
    }

    fn validate_addition_operands(
        left: &LiteralValue,
        right: &LiteralValue,
        operator: &Token,
    ) -> Result<(), RuntimeError> {
        match (left, right) {
            (LiteralValue::Number(_), LiteralValue::Number(_))
            | (LiteralValue::Number(_), LiteralValue::Boolean(_))
            | (LiteralValue::Boolean(_), LiteralValue::Number(_))
            | (LiteralValue::String(_), LiteralValue::String(_)) => Ok(()),
            _ => Err(RuntimeError::new(
                operator.clone(),
                "Operands must be (numbers or booleans) or two strings",
            )),
        }
    }

    fn validate_arithmetic_operands(
        left: &LiteralValue,
        right: &LiteralValue,
        operator: &Token,
    ) -> Result<(), RuntimeError> {
        match (left, right) {
            (LiteralValue::Number(_), LiteralValue::Number(_))
            | (LiteralValue::Number(_), LiteralValue::Boolean(_))
            | (LiteralValue::Boolean(_), LiteralValue::Number(_)) => Ok(()),
            _ => Err(RuntimeError::new(
                operator.clone(),
                "Operands must be numbers or booleans",
            )),
        }
    }

    fn to_number(value: &LiteralValue) -> f64 {
        match value {
            LiteralValue::Number(n) => *n,
            LiteralValue::Boolean(b) => Self::bool_to_number(*b),
            _ => unreachable!(),
        }
    }

    fn bool_to_number(boolean: bool) -> f64 {
        if boolean {
            1.0
        } else {
            0.0
        }
    }

    fn is_truthy(literal: &LiteralValue) -> bool {
        match literal {
            LiteralValue::None => false,
            LiteralValue::Number(number) => *number != 0.0,
            LiteralValue::String(string) => !string.is_empty(),
            LiteralValue::Boolean(boolean) => *boolean,
        }
    }
}

impl ExpressionVisitor for Interpreter {
    type Item = Result<LiteralValue, RuntimeError>;

    fn visit_comma(&self, expr: &mut Comma) -> Self::Item {
        expr.left.accept(&Self)?;
        expr.right.accept(&Self)
    }

    fn visit_ternary(&self, expr: &mut Ternary) -> Self::Item {
        let condition = expr.condition.accept(&Self)?;

        if Self::is_truthy(&condition) {
            Self::evaluate(&mut expr.truth)
        } else {
            Self::evaluate(&mut expr.falsy)
        }
    }

    fn visit_binary(&self, expr: &mut Binary) -> Self::Item {
        let left = expr.left.accept(&Self)?;
        let right = expr.right.accept(&Self)?;

        match &expr.operator.token_type {
            TokenType::Plus => Self::handle_addition(&left, &right, &expr.operator),
            TokenType::Minus | TokenType::Star | TokenType::Slash => {
                Self::handle_arithmetic(&left, &right, &expr.operator)
            }
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => Self::handle_comparison(&left, &right, &expr.operator),
            TokenType::BangEqual => Ok(LiteralValue::Boolean(left != right)),
            TokenType::EqualEqual => Ok(LiteralValue::Boolean(left == right)),
            _ => unreachable!(),
        }
    }

    fn visit_unary(&self, expr: &mut Unary) -> Self::Item {
        let literal = Self::evaluate(&mut expr.right)?;
        let literal = match expr.operator.token_type {
            TokenType::Bang => LiteralValue::Boolean(!Self::is_truthy(&literal)),
            TokenType::Minus => {
                let literal = match literal {
                    LiteralValue::Number(number) => number,
                    LiteralValue::Boolean(boolean) => Self::bool_to_number(boolean),
                    _ => {
                        return Err(RuntimeError::new(
                            expr.operator.clone(),
                            "Unary minus requires number or boolean operand",
                        ))
                    }
                };

                LiteralValue::Number(-literal)
            }
            _ => unreachable!(),
        };

        Ok(literal)
    }

    fn visit_grouping(&self, expr: &mut Grouping) -> Self::Item {
        expr.expression.accept(&Self)
    }

    fn visit_literal(&self, expr: &mut Literal) -> Self::Item {
        Ok(expr.value.clone())
    }
}
