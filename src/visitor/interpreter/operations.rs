use crate::{
    object::Object,
    scanner::{token::Token, token_type::TokenType},
};

use super::RuntimeError;

pub fn handle_addition(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    validate_addition_operands(&left, &right, &operator)?;

    match (left, right) {
        (Object::String(l), Object::String(r)) => return Ok(Object::String(format!("{l}{r}"))),
        _ => {
            let result = to_number(left) + to_number(right);

            return Ok(Object::Number(result));
        }
    };
}

pub fn handle_arithmetic(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    validate_arithmetic_operands(left, right, operator)?;

    let left_number = to_number(left);
    let right_number = to_number(right);
    let value = match operator.token_type {
        TokenType::Minus => left_number - right_number,
        TokenType::Star => left_number * right_number,
        TokenType::Slash => {
            if right_number == 0.0 {
                return Err(RuntimeError::new(
                    operator.clone(),
                    "Not divisible by Zero".to_string(),
                ));
            } else {
                left_number / right_number
            }
        }
        _ => unreachable!(),
    };

    Ok(Object::Number(value))
}

pub fn handle_comparison(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    validate_addition_operands(&left, &right, &operator)?;

    let value = match (left, right) {
        (Object::String(l), Object::String(r)) => match operator.token_type {
            TokenType::Greater => l > r,
            TokenType::GreaterEqual => l >= r,
            TokenType::Less => l < r,
            TokenType::LessEqual => l <= r,
            _ => unreachable!(),
        },
        _ => {
            let l = to_number(left);
            let r = to_number(right);

            match operator.token_type {
                TokenType::Greater => l > r,
                TokenType::GreaterEqual => l >= r,
                TokenType::Less => l < r,
                TokenType::LessEqual => l <= r,
                _ => unreachable!(),
            }
        }
    };

    Ok(Object::Boolean(value))
}

pub fn validate_addition_operands(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<(), RuntimeError> {
    match (left, right) {
        (Object::Number(_), Object::Number(_))
        | (Object::Number(_), Object::Boolean(_))
        | (Object::Boolean(_), Object::Number(_))
        | (Object::String(_), Object::String(_)) => Ok(()),
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be (numbers or booleans) or two strings".to_string(),
        )),
    }
}

pub fn validate_arithmetic_operands(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<(), RuntimeError> {
    match (left, right) {
        (Object::Number(_), Object::Number(_))
        | (Object::Number(_), Object::Boolean(_))
        | (Object::Boolean(_), Object::Number(_)) => Ok(()),
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers or booleans".to_string(),
        )),
    }
}

pub fn to_number(value: &Object) -> f64 {
    match value {
        Object::Number(n) => *n,
        Object::Boolean(b) => bool_to_number(*b),
        _ => unreachable!(),
    }
}

pub fn bool_to_number(boolean: bool) -> f64 {
    if boolean {
        1.0
    } else {
        0.0
    }
}

pub fn is_truthy(literal: &Object) -> bool {
    match literal {
        Object::Undefined => false,
        Object::Number(number) => *number != 0.0,
        Object::String(string) => !string.is_empty(),
        Object::Boolean(boolean) => *boolean,
        Object::Callee(_) => true,
    }
}
