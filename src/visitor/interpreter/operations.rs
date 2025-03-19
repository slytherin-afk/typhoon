use super::RuntimeError;
use crate::{object::Object, scanner::token::Token};

pub fn handle_addition(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    let value = match (left, right) {
        (Object::Number(l), Object::Number(r)) => Object::Number(l + r),
        (Object::Number(l), Object::String(r)) => Object::String(format!("{l}{r}")),
        (Object::Number(l), Object::Boolean(r)) => Object::Number(l + bool_to_number(*r)),
        (Object::String(l), Object::Number(r)) => Object::String(format!("{l}{r}")),
        (Object::String(l), Object::String(r)) => Object::String(format!("{l}{r}")),
        (Object::Boolean(l), Object::Number(r)) => Object::Number(bool_to_number(*l) + r),
        (Object::Boolean(l), Object::Boolean(r)) => {
            Object::Number(bool_to_number(*l) + bool_to_number(*r))
        }
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be (numbers or booleans) or two strings".to_string(),
        ))?,
    };

    Ok(value)
}

pub fn handle_subtraction(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    let value = match (left, right) {
        (Object::Number(l), Object::Number(r)) => Object::Number(l - r),
        (Object::Number(l), Object::Boolean(r)) => Object::Number(l - bool_to_number(*r)),
        (Object::Boolean(l), Object::Number(r)) => Object::Number(bool_to_number(*l) - r),
        (Object::Boolean(l), Object::Boolean(r)) => {
            Object::Number(bool_to_number(*l) - bool_to_number(*r))
        }
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers or booleans".to_string(),
        ))?,
    };

    Ok(value)
}

pub fn handle_multiplication(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    let value = match (left, right) {
        (Object::Number(l), Object::Number(r)) => Object::Number(l * r),
        (Object::Number(l), Object::Boolean(r)) => Object::Number(l * bool_to_number(*r)),
        (Object::Boolean(l), Object::Number(r)) => Object::Number(bool_to_number(*l) * r),
        (Object::Boolean(l), Object::Boolean(r)) => {
            Object::Number(bool_to_number(*l) * bool_to_number(*r))
        }
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers or booleans".to_string(),
        ))?,
    };

    Ok(value)
}

pub fn handle_division(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    let divide = |l, r| {
        if r == 0.0 {
            Err(RuntimeError::new(
                operator.clone(),
                "Divide by zero".to_string(),
            ))
        } else {
            Ok(l / r)
        }
    };

    let value = match (left, right) {
        (Object::Number(l), Object::Number(r)) => Object::Number(divide(*l, *r)?),
        (Object::Number(l), Object::Boolean(r)) => Object::Number(divide(*l, bool_to_number(*r))?),
        (Object::Boolean(l), Object::Number(r)) => Object::Number(divide(bool_to_number(*l), *r)?),
        (Object::Boolean(l), Object::Boolean(r)) => {
            Object::Number(divide(bool_to_number(*l), bool_to_number(*r))?)
        }
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers or booleans".to_string(),
        ))?,
    };

    Ok(value)
}

pub fn handle_less_than(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    match (left, right) {
        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l < r)),
        (Object::Number(l), Object::Boolean(r)) => Ok(Object::Boolean(*l < bool_to_number(*r))),
        (Object::Boolean(l), Object::Number(r)) => Ok(Object::Boolean(bool_to_number(*l) < *r)),
        (Object::Boolean(l), Object::Boolean(r)) => {
            Ok(Object::Boolean(bool_to_number(*l) < bool_to_number(*r)))
        }
        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l < r)),
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers, booleans, or strings".to_string(),
        )),
    }
}

pub fn handle_greater_than(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    match (left, right) {
        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l > r)),
        (Object::Number(l), Object::Boolean(r)) => Ok(Object::Boolean(*l > bool_to_number(*r))),
        (Object::Boolean(l), Object::Number(r)) => Ok(Object::Boolean(bool_to_number(*l) > *r)),
        (Object::Boolean(l), Object::Boolean(r)) => {
            Ok(Object::Boolean(bool_to_number(*l) > bool_to_number(*r)))
        }
        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l > r)),
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers, booleans, or strings".to_string(),
        )),
    }
}

pub fn handle_less_than_equal(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    match (left, right) {
        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l <= r)),
        (Object::Number(l), Object::Boolean(r)) => Ok(Object::Boolean(*l <= bool_to_number(*r))),
        (Object::Boolean(l), Object::Number(r)) => Ok(Object::Boolean(bool_to_number(*l) <= *r)),
        (Object::Boolean(l), Object::Boolean(r)) => {
            Ok(Object::Boolean(bool_to_number(*l) <= bool_to_number(*r)))
        }
        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l <= r)),
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers, booleans, or strings".to_string(),
        )),
    }
}

pub fn handle_greater_than_equal(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    match (left, right) {
        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l >= r)),
        (Object::Number(l), Object::Boolean(r)) => Ok(Object::Boolean(*l >= bool_to_number(*r))),
        (Object::Boolean(l), Object::Number(r)) => Ok(Object::Boolean(bool_to_number(*l) >= *r)),
        (Object::Boolean(l), Object::Boolean(r)) => {
            Ok(Object::Boolean(bool_to_number(*l) >= bool_to_number(*r)))
        }
        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l >= r)),
        _ => Err(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers, booleans, or strings".to_string(),
        )),
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
        Object::Callable(_) => true,
    }
}
