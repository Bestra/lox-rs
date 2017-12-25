use token::{Token, TokenLiteral};
#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Unary {
        left: Box<Expr>,
        operator: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: TokenLiteral,
    },
}

pub fn pretty_print(e: Expr) -> String {
    match e {
        Expr::Binary {
            operator: o,
            left: l,
            right: r,
        } => parenthesize(&o.lexeme, vec![*l, *r]),
        Expr::Unary {
            operator: o,
            left: l,
        } => parenthesize(&o.lexeme, vec![*l]),
        Expr::Grouping { expression: o } => parenthesize("group", vec![*o]),
        Expr::Literal { value: v } => format!("{:?}", v),
    }
}

fn parenthesize(tag: &str, exprs: Vec<Expr>) -> String {
    let inner_str: Vec<String> = exprs.into_iter().map(|e| pretty_print(e)).collect();
    format!("{} {}", tag, inner_str.join(" "))
}
