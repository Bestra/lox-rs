use token::{LoxValue, Token};
#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Unary {
        right: Box<Expr>,
        operator: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LoxValue,
    },
}

pub fn pretty_print(e: &Expr) -> String {
    match *e {
        Expr::Binary {
            operator: ref o,
            left: ref l,
            right: ref r,
        } => parenthesize(&o.lexeme, vec![&**l, &**r]),
        Expr::Unary {
            operator: ref o,
            right: ref r,
        } => parenthesize(&o.lexeme, vec![&**r]),
        Expr::Grouping { expression: ref o } => parenthesize("group", vec![&**o]),
        Expr::Literal { value: ref v } => format!("{:?}", v),
    }
}

fn parenthesize(tag: &str, exprs: Vec<&Expr>) -> String {
    let inner_str: Vec<String> = exprs.into_iter().map(|e| pretty_print(e)).collect();
    format!("({} {})", tag, inner_str.join(" "))
}
