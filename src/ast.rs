use token::{LoxValue, Token};
use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};
#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
        token: Token,
    },
    Literal {
        value: LoxValue,
        token: Token,
    },
    Logical {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Unary {
        right: Box<Expr>,
        operator: Token,
    },
    Variable {
        name: Token,
    },
}

impl Expr {
    fn token(&self) -> &Token {
        match *self {
            Expr::Assign { ref name, .. } | Expr::Variable { ref name, .. } => name,
            Expr::Call { ref paren, .. } => paren,
            Expr::Grouping { ref token, .. } | Expr::Literal { ref token, .. } => token,
            Expr::Binary { ref operator, .. }
            | Expr::Logical { ref operator, .. }
            | Expr::Unary { ref operator, .. } => operator,
        }
    }

    pub fn string_id(&self) -> String {
        let t = self.token();
        format!("{:?} {:?}", t.token_type, t.position)
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Expr) -> bool {
        self.token() == other.token()
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token().hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block {
        statements: Vec<Statement>,
    },
    Expression {
        expression: Box<Expr>,
    },
    Function(FunctionDeclaration),
    If {
        condition: Box<Expr>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Print {
        expression: Box<Expr>,
    },
    Return {
        keyword: Token,
        value: Option<Box<Expr>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Statement>,
    },
    Var {
        name: Token,
        initializer: Option<Box<Expr>>,
    },
}

pub struct Program {
    pub statements: Vec<Statement>,
}

// pub trait AstPrint {
//     fn pretty_print(&self) -> String;
// }

// impl AstPrint for Program {
//     fn pretty_print(&self) -> String {
//         let inner_str: Vec<String> = self.statements.iter().map(|s| s.pretty_print()).collect();
//         format!("({} {})", "program", inner_str.join(" "))
//     }
// }

// impl AstPrint for Statement {
//     fn pretty_print(&self) -> String {
//         match *self {
//             Statement::Expression { ref expression } => {
//                 format!("({} {})", "expr", expression.pretty_print())
//             }

//             Statement::Print { ref expression } => {
//                 format!("({} {})", "print", expression.pretty_print())
//             }

//             Statement::Block { ref statements } => {
//                 let inner_str: Vec<String> = statements.iter().map(|s| s.pretty_print()).collect();
//                 format!("({} {})", "block", inner_str.join(" "))
//             }

//             Statement::If {
//                 ref condition,
//                 ref then_branch,
//                 ref else_branch,
//             } => {
//                 let else_str = match *else_branch {
//                     Some(ref e) => e.pretty_print(),
//                     None => "".to_string(),
//                 };

//                 format!(
//                     "({} {} {} {})",
//                     "if",
//                     condition.pretty_print(),
//                     then_branch.pretty_print(),
//                     else_str
//                 )
//             }

//             Statement::While {
//                 ref condition,
//                 ref body,
//             } => format!(
//                 "({} {} {})",
//                 "while",
//                 condition.pretty_print(),
//                 body.pretty_print()
//             ),

//             Statement::Var {
//                 ref name,
//                 ref initializer,
//             } => {
//                 let var = match *initializer {
//                     Some(ref v) => format!("{}", v.pretty_print()),
//                     None => "nil".to_string(),
//                 };
//                 format!("({} {} {})", "def_var", name.lexeme, var)
//             }
//         }
//     }
// }

// impl AstPrint for Expr {
//     fn pretty_print(&self) -> String {
//         match *self {
//             Expr::Assign {
//                 ref name,
//                 ref value,
//             } => format!("({} {} {})", "set_var", name.lexeme, value.pretty_print()),
//             Expr::Binary {
//                 operator: ref o,
//                 left: ref l,
//                 right: ref r,
//             }
//             | Expr::Logical {
//                 operator: ref o,
//                 left: ref l,
//                 right: ref r,
//             } => format!("({} {} {})", o.lexeme, l.pretty_print(), r.pretty_print()),
//             Expr::Unary {
//                 operator: ref o,
//                 right: ref r,
//             } => format!("({} {})", o.lexeme, r.pretty_print()),
//             Expr::Call {
//                 ref callee,
//                 ref paren,
//                 ref arguments,
//             }=> {
//                 let args: Vec<String> = arguments.iter().map(|s| s.pretty_print()).collect();
//                 format!("({} {} {})", "call", callee.pretty_print(), args.join(" "))
//             }
//             Expr::Grouping { expression: ref r } => format!("({} {})", "group", r.pretty_print()),
//             Expr::Literal { value: ref v } => format!("{:?}", v),
//             Expr::Variable { name: ref n } => format!("{}", n.lexeme),
//         }
//     }
// }
