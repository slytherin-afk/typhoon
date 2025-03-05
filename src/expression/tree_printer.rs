use super::{
    binary::Binary,
    comma::Comma,
    grouping::Grouping,
    literal::{Literal, LiteralValue},
    ternary::Ternary,
    unary::Unary,
    Expression, ExpressionVisitor,
};

pub struct TreePrinter;

impl TreePrinter {
    pub fn print(expression: &mut Expression) -> String {
        expression.accept(&TreePrinter)
    }

    fn parenthesize<T: ExpressionVisitor<String>>(
        visitor: &T,
        name: &str,
        expressions: &mut Vec<&mut Expression>,
    ) -> String {
        let mut builder = format!("({name}");

        expressions.iter_mut().for_each(|ex| {
            let expression = ex.accept(visitor);
            builder = format!("{builder} {expression}");
        });

        builder.push(')');
        builder
    }
}

impl ExpressionVisitor<String> for TreePrinter {
    fn visit_comma(&self, expr: &mut Comma) -> String {
        let mut expressions = vec![&mut expr.left, &mut expr.right];

        Self::parenthesize(self, ",", &mut expressions)
    }

    fn visit_ternary(&self, expr: &mut Ternary) -> String {
        let mut expressions = vec![&mut expr.condition, &mut expr.truth, &mut expr.falsy];

        Self::parenthesize(self, "?:", &mut expressions)
    }

    fn visit_binary(&self, expr: &mut Binary) -> String {
        let mut expressions = vec![&mut expr.left, &mut expr.right];

        Self::parenthesize(self, &expr.operator.lexeme, &mut expressions)
    }

    fn visit_unary(&self, expr: &mut Unary) -> String {
        let mut expressions = vec![&mut expr.right];

        Self::parenthesize(self, &expr.operator.lexeme, &mut expressions)
    }

    fn visit_grouping(&self, expr: &mut Grouping) -> String {
        let mut expressions = vec![&mut expr.expression];

        Self::parenthesize(self, "group", &mut expressions)
    }

    fn visit_literal(&self, expr: &mut Literal) -> String {
        match expr.value {
            LiteralValue::True => "True".to_string(),
            LiteralValue::False => "String".to_string(),
            LiteralValue::Number(number) => number.to_string(),
            LiteralValue::String(value) => value.to_string(),
            LiteralValue::None => "None".to_string(),
        }
    }
}
