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
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Statement {
    Expression {
        expression: Box<Expr>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Box<Expr>>,
    },
    Block {
        statements: Vec<Statement>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
}

pub struct Program {
    pub statements: Vec<Statement>,
}

pub trait AstPrint {
    fn pretty_print(&self) -> String;
}

impl AstPrint for Program {
    fn pretty_print(&self) -> String {
        let inner_str: Vec<String> = self.statements.iter().map(|s| s.pretty_print()).collect();
        format!("({} {})", "program", inner_str.join(" "))
    }
}

impl AstPrint for Statement {
    fn pretty_print(&self) -> String {
        match *self {
            Statement::Expression { ref expression } => {
                format!("({} {})", "expr", expression.pretty_print())
            }

            Statement::Print { ref expression } => {
                format!("({} {})", "print", expression.pretty_print())
            }

            Statement::Block { ref statements } => {
                let inner_str: Vec<String> = statements.iter().map(|s| s.pretty_print()).collect();
                format!("({} {})", "block", inner_str.join(" "))
            }

            Statement::If {
                ref condition,
                ref then_branch,
                ref else_branch,
            } => {
                let else_str = match *else_branch {
                    Some(ref e) => e.pretty_print(),
                    None => "".to_string(),
                };

                format!(
                    "({} {} {} {})",
                    "if",
                    condition.pretty_print(),
                    then_branch.pretty_print(),
                    else_str
                )
            }

            Statement::Var {
                ref name,
                ref initializer,
            } => {
                let var = match *initializer {
                    Some(ref v) => format!("{}", v.pretty_print()),
                    None => "nil".to_string(),
                };
                format!("({} {} {})", "def_var", name.lexeme, var)
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
            } => format!("({} {} {})", o.lexeme, l.pretty_print(), r.pretty_print()),
            Expr::Unary {
                operator: ref o,
                right: ref r,
            } => format!("({} {})", o.lexeme, r.pretty_print()),
            Expr::Grouping { expression: ref r } => format!("({} {})", "group", r.pretty_print()),
            Expr::Literal { value: ref v } => format!("{:?}", v),
            Expr::Variable { name: ref n } => format!("{}", n.lexeme),
            Expr::Assign {
                ref name,
                ref value,
            } => format!("({} {} {})", "set_var", name.lexeme, value.pretty_print()),
        }
    }
}
