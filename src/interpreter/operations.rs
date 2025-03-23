use crate::{errors::RuntimeError, object::Object, token::Token, utils::bool_to_number};

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
        _ => Err(RuntimeError {
            token: operator.clone(),
            message: String::from("Operands must be (numbers or booleans) or two strings"),
        })?,
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
        _ => Err(RuntimeError {
            token: operator.clone(),
            message: String::from("Operands must be numbers or booleans"),
        })?,
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
        _ => Err(RuntimeError {
            token: operator.clone(),
            message: String::from("Operands must be numbers or booleans"),
        })?,
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
            Err(RuntimeError {
                token: operator.clone(),
                message: String::from("Divide by zero"),
            })
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
        _ => Err(RuntimeError {
            token: operator.clone(),
            message: String::from("Operands must be numbers or booleans"),
        })?,
    };

    Ok(value)
}

pub fn handle_modulus(
    left: &Object,
    right: &Object,
    operator: &Token,
) -> Result<Object, RuntimeError> {
    let value = match (left, right) {
        (Object::Number(l), Object::Number(r)) => Object::Number(l % r),
        (Object::Number(l), Object::Boolean(r)) => Object::Number(l % bool_to_number(*r)),
        (Object::Boolean(l), Object::Number(r)) => Object::Number(bool_to_number(*l) % r),
        (Object::Boolean(l), Object::Boolean(r)) => {
            Object::Number(bool_to_number(*l) % bool_to_number(*r))
        }
        _ => Err(RuntimeError {
            token: operator.clone(),
            message: String::from("Operands must be numbers or booleans"),
        })?,
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
        _ => Err(RuntimeError {
            token: operator.clone(),
            message: String::from("Operands must be numbers, booleans, or strings"),
        }),
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
        _ => Err(RuntimeError {
            token: operator.clone(),
            message: String::from("Operands must be numbers, booleans, or strings"),
        }),
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
        _ => Err(RuntimeError {
            token: operator.clone(),
            message: String::from("Operands must be numbers, booleans, or strings"),
        }),
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
        _ => Err(RuntimeError {
            token: operator.clone(),
            message: String::from("Operands must be numbers, booleans, or strings"),
        }),
    }
}
