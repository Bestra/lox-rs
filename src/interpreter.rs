use ast::{Expr, Program, Statement};
use std::fmt;
use token::{LoxValue, TokenType, Token};

pub struct RuntimeError {
    token: Token,
    message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime error at {:?}: {}", self.token, self.message)
    }
}

pub fn interpret(e: Program) -> Result<bool, RuntimeError> {
    for s in e.statements {
        execute(s)?;
    }

    Ok(true)
}

fn execute(s: Statement) -> Result<LoxValue, RuntimeError> {
    match s {
        Statement::Expression { expression } => {evaluate(*expression)},
        Statement::Print { expression } => {
            let val =  evaluate(*expression)?;
            println!("{}", val);
            Ok(val)
        },
    }
}

fn evaluate(e: Expr) -> Result<LoxValue, RuntimeError> {
    match e {
        Expr::Literal { value } => Ok(value),
        Expr::Grouping { expression } => evaluate(*expression),
        Expr::Unary { right, operator } => {
            let r_val = evaluate(*right)?;
            match operator.token_type {
                TokenType::Bang => Ok(LoxValue::Bool(!is_truthy(&r_val))),
                TokenType::Minus => match r_val {
                    LoxValue::Number(n) => Ok(LoxValue::Number(-n)),
                    _ => Ok(LoxValue::Nil),
                },
                _ => Ok(LoxValue::Nil),
            }
        }
        Expr::Binary {
            left,
            right,
            operator,
        } => {
            let l_val = evaluate(*left)?;
            let r_val = evaluate(*right)?;
            let _ok = check_number_operands(&operator, &l_val, &r_val)?;

            println!("number check: {:?}", _ok);
            match (operator.token_type.clone(), l_val, r_val) {
                // Numerical operations
                (TokenType::Minus, LoxValue::Number(a), LoxValue::Number(b)) => {
                    Ok(LoxValue::Number(a - b))
                }
                (TokenType::Slash, LoxValue::Number(a), LoxValue::Number(b)) => {
                    Ok(LoxValue::Number(a / b))
                }
                (TokenType::Star, LoxValue::Number(a), LoxValue::Number(b)) => {
                    Ok(LoxValue::Number(a * b))
                }
                (TokenType::Plus, LoxValue::Number(a), LoxValue::Number(b)) => {
                    Ok(LoxValue::Number(a + b))
                }
                //Comparison
                (TokenType::Greater, LoxValue::Number(a), LoxValue::Number(b)) => {
                    Ok(LoxValue::Bool(a > b))
                }
                (TokenType::GreaterEqual, LoxValue::Number(a), LoxValue::Number(b)) => {
                    Ok(LoxValue::Bool(a >= b))
                }
                (TokenType::Less, LoxValue::Number(a), LoxValue::Number(b)) => {
                    Ok(LoxValue::Bool(a < b))
                }
                (TokenType::LessEqual, LoxValue::Number(a), LoxValue::Number(b)) => {
                    Ok(LoxValue::Bool(a <= b))
                }

                (TokenType::EqualEqual, a, b) => Ok(LoxValue::Bool(is_equal(&a, &b))),
                (TokenType::BangEqual, a, b) => Ok(LoxValue::Bool(!is_equal(&a, &b))),

                // String Concat
                (TokenType::Plus, LoxValue::String(a), LoxValue::String(b)) => {
                    Ok(LoxValue::String(format!("{}{}", a, b)))
                }
                (_, a, b) => Err(RuntimeError {token: operator.clone(), message: format!("There was some problem applying {:?} to operands {:?} and {:?}", operator, a, b)}),
            }
        }
    }
}

fn check_number_operands(t: &Token, a: &LoxValue, b: &LoxValue) -> Result<bool, RuntimeError>{
    match t.token_type {
        TokenType::Plus |
        TokenType::Minus |
        TokenType::Slash |
        TokenType::Star |
        TokenType::Greater |
        TokenType::GreaterEqual |
        TokenType::Less |
        TokenType::LessEqual => {
            match (a, b) {
                (&LoxValue::Number(_), &LoxValue::Number(_)) => Ok(true),
                (_, _) => Err(RuntimeError {token: t.clone(), message: format!("Operands {:?} and {:?} must both be numbers.", a, b)})
            }
        }
        _ => Ok(true)
    }
}
fn is_equal(a: &LoxValue, b: &LoxValue) -> bool {
    a.eq(b)
}
fn is_truthy(e: &LoxValue) -> bool {
    match *e {
        LoxValue::Nil => false,
        LoxValue::Bool(b) => b,
        _ => true,
    }
}
