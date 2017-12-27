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

#[derive(Debug)]
pub enum Statement {
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
}

pub struct Program {
    pub statements: Vec<Statement>,
}

pub trait AstPrint {
    fn pretty_print(&self) -> String;
}

impl AstPrint for Program {
    fn pretty_print(&self) -> String {
        let inner_str: Vec<String>  = self.statements.iter().map(|s| s.pretty_print()).collect();
        format!("({} {})", "program", inner_str.join(" "))
    }
}

impl AstPrint for Statement {
    fn pretty_print(&self) -> String {
        match *self {
            Statement::Expression {ref expression} => {
                format!("({} {})", "expr", expression.pretty_print())
            }

            Statement::Print {ref expression} => {
                format!("({} {})", "print", expression.pretty_print())
            }
        }
    }
}

impl AstPrint for Expr {
    fn pretty_print(&self) -> String {
        match *self {
            Expr::Binary {
                operator: ref o,
                left: ref l,
                right: ref r,
            } => {
                format!("({} {} {})", &o.lexeme, l.pretty_print(), r.pretty_print())
            },
            Expr::Unary {
                operator: ref o,
                right: ref r,
            } => {
                format!("({} {})", &o.lexeme, r.pretty_print())
            }
            Expr::Grouping { expression: ref r } => {
                format!("({} {})", "group", r.pretty_print())
            }
            Expr::Literal { value: ref v } => format!("{:?}", v),
        }
    }
}
