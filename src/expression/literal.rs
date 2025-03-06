use std::fmt;

#[derive(Clone, PartialEq)]
pub enum LiteralValue {
    None,
    Number(f64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            LiteralValue::None => "none",
            LiteralValue::Number(n) => &n.to_string(),
            LiteralValue::String(s) => &s.to_string(),
            LiteralValue::Boolean(b) => &b.to_string(),
        };

        write!(f, "{value}")
    }
}

pub struct Literal {
    pub value: LiteralValue,
}
