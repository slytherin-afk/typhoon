use crate::expression::{
    assignment::Assignment, binary::Binary, comma::Comma, grouping::Grouping, literal::Literal,
    ternary::Ternary, unary::Unary, variable::Variable, Expression,
};

use super::ExpressionVisitor;

pub struct TreeNode {
    name: String,
    children: Vec<TreeNode>,
}

pub struct PrettyAstPrinter;

impl PrettyAstPrinter {
    pub fn print(expression: &mut Expression) -> String {
        let tree = expression.accept(&mut Self);
        Self::print_tree(&tree, &String::new(), true)
    }

    fn print_tree(node: &TreeNode, prefix: &str, is_last: bool) -> String {
        let mut output = String::new();
        let branch = if is_last { "└── " } else { "├── " };

        output.push_str(&format!("{}{}{}\n", prefix, branch, node.name));

        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let count = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            output.push_str(&Self::print_tree(child, &new_prefix, i == count - 1));
        }

        output
    }

    fn to_tree_node(&mut self, name: &str, expressions: &mut Vec<&mut Expression>) -> TreeNode {
        let mut node = TreeNode {
            name: name.to_string(),
            children: Vec::new(),
        };

        for expr in expressions.iter_mut() {
            node.children.push(expr.accept(self));
        }

        node
    }
}

impl ExpressionVisitor for PrettyAstPrinter {
    type Item = TreeNode;

    fn visit_comma(&mut self, expr: &mut Comma) -> Self::Item {
        let mut expressions = vec![&mut expr.right, &mut expr.left];
        self.to_tree_node(",", &mut expressions)
    }

    fn visit_ternary(&mut self, expr: &mut Ternary) -> Self::Item {
        let mut expressions = vec![&mut expr.falsy, &mut expr.truth, &mut expr.condition];
        self.to_tree_node("?:", &mut expressions)
    }

    fn visit_binary(&mut self, expr: &mut Binary) -> Self::Item {
        let mut expressions = vec![&mut expr.right, &mut expr.left];
        self.to_tree_node(&expr.operator.lexeme, &mut expressions)
    }

    fn visit_unary(&mut self, expr: &mut Unary) -> Self::Item {
        let mut expressions = vec![&mut expr.right];
        self.to_tree_node(&expr.operator.lexeme, &mut expressions)
    }

    fn visit_grouping(&mut self, expr: &mut Grouping) -> Self::Item {
        let mut expressions = vec![&mut expr.expression];
        self.to_tree_node("group", &mut expressions)
    }

    fn visit_literal(&mut self, expr: &mut Literal) -> Self::Item {
        TreeNode {
            name: expr.value.to_string(),
            children: vec![],
        }
    }

    fn visit_variable(&mut self, expr: &mut Variable) -> Self::Item {
        TreeNode {
            name: expr.name.lexeme.to_string(),
            children: vec![],
        }
    }

    fn visit_assignment(
        &mut self,
        expr: &mut crate::expression::assignment::Assignment,
    ) -> Self::Item {
        let mut expressions = vec![&mut expr.expression];
        self.to_tree_node(&expr.name.lexeme, &mut expressions)
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(expression: &mut Expression) -> String {
        expression.accept(&mut Self)
    }

    fn parenthesize(&mut self, name: &str, expressions: &mut Vec<&mut Expression>) -> String {
        let mut builder = format!("({name}");

        expressions.iter_mut().for_each(|ex| {
            let expression = ex.accept(self);
            builder = format!("{builder} {expression}");
        });

        builder.push(')');
        builder
    }
}

impl ExpressionVisitor for AstPrinter {
    type Item = String;

    fn visit_comma(&mut self, expr: &mut Comma) -> Self::Item {
        let mut expressions = vec![&mut expr.left, &mut expr.right];

        self.parenthesize(",", &mut expressions)
    }

    fn visit_ternary(&mut self, expr: &mut Ternary) -> Self::Item {
        let mut expressions = vec![&mut expr.condition, &mut expr.truth, &mut expr.falsy];

        self.parenthesize("?:", &mut expressions)
    }

    fn visit_binary(&mut self, expr: &mut Binary) -> Self::Item {
        let mut expressions = vec![&mut expr.left, &mut expr.right];

        self.parenthesize(&expr.operator.lexeme, &mut expressions)
    }

    fn visit_unary(&mut self, expr: &mut Unary) -> Self::Item {
        let mut expressions = vec![&mut expr.right];

        self.parenthesize(&expr.operator.lexeme, &mut expressions)
    }

    fn visit_grouping(&mut self, expr: &mut Grouping) -> Self::Item {
        let mut expressions = vec![&mut expr.expression];

        self.parenthesize("group", &mut expressions)
    }

    fn visit_literal(&mut self, expr: &mut Literal) -> Self::Item {
        expr.value.to_string()
    }

    fn visit_variable(&mut self, expr: &mut Variable) -> Self::Item {
        expr.name.lexeme.to_string()
    }

    fn visit_assignment(&mut self, expr: &mut Assignment) -> Self::Item {
        let mut expressions = vec![&mut expr.expression];

        self.parenthesize(&expr.name.lexeme, &mut expressions)
    }
}
