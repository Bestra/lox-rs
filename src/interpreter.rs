use ast::Expr;
use token::{LoxValue, TokenType};

pub fn interpret(e: Expr) -> LoxValue {
    evaluate(e)
}

fn evaluate(e: Expr) -> LoxValue {
    match e {
        Expr::Literal { value } => value,
        Expr::Grouping { expression } => evaluate(*expression),
        Expr::Unary { right, operator } => {
            let r_val = evaluate(*right);
            match operator.token_type {
                TokenType::Bang => LoxValue::Bool(!is_truthy(r_val)),
                TokenType::Minus => match r_val {
                    LoxValue::Number(n) => LoxValue::Number(-n),
                    _ => LoxValue::Nil,
                },
                _ => LoxValue::Nil,
            }
        }
        Expr::Binary {
            left,
            right,
            operator,
        } => {
            let l_val = evaluate(*left);
            let r_val = evaluate(*right);

            match (operator.token_type, l_val, r_val) {
                // Numerical operations
                (TokenType::Minus, LoxValue::Number(a), LoxValue::Number(b)) => {
                    LoxValue::Number(a - b)
                }
                (TokenType::Slash, LoxValue::Number(a), LoxValue::Number(b)) => {
                    LoxValue::Number(a / b)
                }
                (TokenType::Star, LoxValue::Number(a), LoxValue::Number(b)) => {
                    LoxValue::Number(a * b)
                }
                (TokenType::Plus, LoxValue::Number(a), LoxValue::Number(b)) => {
                    LoxValue::Number(a + b)
                }
                //Comparison
                (TokenType::Greater, LoxValue::Number(a), LoxValue::Number(b)) => {
                    LoxValue::Bool(a > b)
                }
                (TokenType::GreaterEqual, LoxValue::Number(a), LoxValue::Number(b)) => {
                    LoxValue::Bool(a >= b)
                }
                (TokenType::Less, LoxValue::Number(a), LoxValue::Number(b)) => {
                    LoxValue::Bool(a < b)
                }
                (TokenType::LessEqual, LoxValue::Number(a), LoxValue::Number(b)) => {
                    LoxValue::Bool(a <= b)
                }

                (TokenType::EqualEqual, a, b) => LoxValue::Bool(is_equal(a, b)),
                (TokenType::BangEqual, a, b) => LoxValue::Bool(!is_equal(a, b)),

                // String Concat
                (TokenType::Plus, LoxValue::String(a), LoxValue::String(b)) => {
                    LoxValue::String(format!("{}{}", a, b))
                }
                _ => LoxValue::Nil,
            }
        }
    }
}

fn is_equal(a: LoxValue, b: LoxValue) -> bool {
    a.eq(&b)
    // (LoxValue::Nil, LoxValue::Nil) => LoxValue::Bool(true),
    // (LoxValue::Nil, _) => LoxValue::Bool(false),
    // (_, LoxValue::Nil) => LoxValue::Bool(false),
}
fn is_truthy(e: LoxValue) -> bool {
    match e {
        LoxValue::Nil => false,
        LoxValue::Bool(b) => b,
        _ => true,
    }
}
