use typhoon::{
    expression::{
        binary::Binary, grouping::Grouping, literal::Literal, tree_printer::TreePrinter,
        unary::Unary, Expression,
    },
    scanner::{
        token::{LiteralType, Token},
        token_type::TokenType,
    },
};

fn main() {
    let mut expression = Expression::Binary(Box::new(Binary {
        left: Expression::Unary(Box::new(Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Expression::Literal(Box::new(Literal {
                value: Token {
                    token_type: TokenType::NumberLiteral,
                    lexeme: "123".to_string(),
                    literal: Some(LiteralType::Number(123.0)),
                    line: 1,
                },
            })),
        })),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Expression::Grouping(Box::new(Grouping {
            expression: Expression::Literal(Box::new(Literal {
                value: Token {
                    token_type: TokenType::NumberLiteral,
                    lexeme: "45.67".to_string(),
                    literal: Some(LiteralType::Number(45.67)),
                    line: 1,
                },
            })),
        })),
    }));

    println!("{}", TreePrinter::print(&mut expression));
}
