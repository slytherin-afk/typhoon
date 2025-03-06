use super::{
    binary::Binary, comma::Comma, grouping::Grouping, literal::Literal, ternary::Ternary,
    unary::Unary, Expression, ExpressionVisitor,
};

pub struct TreeNode {
    name: String,
    children: Vec<TreeNode>,
}

pub struct PrettyAstPrinter;

impl PrettyAstPrinter {
    pub fn print(expression: &mut Expression) -> String {
        let tree = expression.accept(&Self);
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

    fn to_tree_node<T: ExpressionVisitor<Item = TreeNode>>(
        visitor: &T,
        name: &str,
        expressions: &mut Vec<&mut Expression>,
    ) -> TreeNode {
        let mut node = TreeNode {
            name: name.to_string(),
            children: Vec::new(),
        };

        for expr in expressions.iter_mut() {
            node.children.push(expr.accept(visitor));
        }

        node
    }
}

impl ExpressionVisitor for PrettyAstPrinter {
    type Item = TreeNode;

    fn visit_comma(&self, expr: &mut Comma) -> Self::Item {
        let mut expressions = vec![&mut expr.right, &mut expr.left];
        Self::to_tree_node(self, ",", &mut expressions)
    }

    fn visit_ternary(&self, expr: &mut Ternary) -> Self::Item {
        let mut expressions = vec![&mut expr.falsy, &mut expr.truth, &mut expr.condition];
        Self::to_tree_node(self, "?:", &mut expressions)
    }

    fn visit_binary(&self, expr: &mut Binary) -> Self::Item {
        let mut expressions = vec![&mut expr.right, &mut expr.left];
        Self::to_tree_node(self, &expr.operator.lexeme, &mut expressions)
    }

    fn visit_unary(&self, expr: &mut Unary) -> Self::Item {
        let mut expressions = vec![&mut expr.right];
        Self::to_tree_node(self, &expr.operator.lexeme, &mut expressions)
    }

    fn visit_grouping(&self, expr: &mut Grouping) -> Self::Item {
        let mut expressions = vec![&mut expr.expression];
        Self::to_tree_node(self, "group", &mut expressions)
    }

    fn visit_literal(&self, expr: &mut Literal) -> Self::Item {
        TreeNode {
            name: expr.value.to_string(),
            children: vec![],
        }
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(expression: &mut Expression) -> String {
        expression.accept(&Self)
    }

    fn parenthesize<T: ExpressionVisitor<Item = String>>(
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

impl ExpressionVisitor for AstPrinter {
    type Item = String;

    fn visit_comma(&self, expr: &mut Comma) -> Self::Item {
        let mut expressions = vec![&mut expr.left, &mut expr.right];

        Self::parenthesize(self, ",", &mut expressions)
    }

    fn visit_ternary(&self, expr: &mut Ternary) -> Self::Item {
        let mut expressions = vec![&mut expr.condition, &mut expr.truth, &mut expr.falsy];

        Self::parenthesize(self, "?:", &mut expressions)
    }

    fn visit_binary(&self, expr: &mut Binary) -> Self::Item {
        let mut expressions = vec![&mut expr.left, &mut expr.right];

        Self::parenthesize(self, &expr.operator.lexeme, &mut expressions)
    }

    fn visit_unary(&self, expr: &mut Unary) -> Self::Item {
        let mut expressions = vec![&mut expr.right];

        Self::parenthesize(self, &expr.operator.lexeme, &mut expressions)
    }

    fn visit_grouping(&self, expr: &mut Grouping) -> Self::Item {
        let mut expressions = vec![&mut expr.expression];

        Self::parenthesize(self, "group", &mut expressions)
    }

    fn visit_literal(&self, expr: &mut Literal) -> Self::Item {
        expr.value.to_string()
    }
}
