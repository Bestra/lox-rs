use ast::{Expr, Program, Statement};
use std::fmt;
use token::{LoxValue, Token, TokenType};
use lox_callable::{Clock, LoxCallable, LoxFunction};
use environment::Environment;
use std::rc::Rc;

pub enum Error {
    Return(LoxValue),
    RuntimeError { token: Token, message: String },
}

pub struct ReturnValue(LoxValue);

type IResult<T> = Result<T, Error>;

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut env = Environment::new();
        env.define("clock".to_string(), LoxValue::Fn(Rc::new(Clock)));
        Interpreter { environment: env }
    }

    pub fn interpret(&mut self, e: Program) -> IResult<()> {
        for s in e.statements {
            self.execute(&s)?;
        }

        Ok(())
    }

    fn execute(&mut self, s: &Statement) -> IResult<()> {
        match *s {
            Statement::Block { ref statements } => {
                self.environment.push();
                let ret = self.execute_block(statements);
                self.environment.pop();
                ret
            }
            Statement::Expression { ref expression } => {
                self.evaluate(&*expression)?;
                Ok(())
            }
            Statement::Function(ref stmt) => {
                let c = stmt.clone();
                let function = LoxFunction::new(c, &self.environment);
                self.environment
                    .define(stmt.name.lexeme.clone(), LoxValue::Fn(Rc::new(function)));
                Ok(())
            }
            Statement::If {
                ref condition,
                ref then_branch,
                ref else_branch,
            } => {
                if is_truthy(&self.evaluate(&*condition)?) {
                    self.execute(&*then_branch)?;
                } else if let Some(ref b) = *else_branch {
                    self.execute(&*b)?;
                }
                Ok(())
            }
            Statement::Print { ref expression } => {
                let val = self.evaluate(&*expression)?;
                println!("{}", val);
                Ok(())
            }
            Statement::Return {
                ref keyword,
                ref value,
            } => {
                let val = match *value {
                    Some(ref v) => self.evaluate(&*v)?,
                    None => LoxValue::Nil,
                };
                Err(Error::Return(val))
            }
            Statement::Var {
                ref name,
                ref initializer,
            } => {
                let val = match *initializer {
                    Some(ref v) => self.evaluate(&*v)?,
                    None => LoxValue::Nil,
                };

                self.environment.define(name.lexeme.clone(), val.clone());
                Ok(())
            }
            Statement::While {
                ref condition,
                ref body,
            } => {
                while is_truthy(&self.evaluate(&*condition)?) {
                    self.execute(&*body)?;
                }
                Ok(())
            }
        }
    }

    pub fn execute_block(&mut self, statements: &[Statement]) -> IResult<()> {
        for s in statements {
            match self.execute(s) {
                Ok(_r) => (),
                Err(r) => {
                    return Err(r);
                }
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, e: &Expr) -> IResult<LoxValue> {
        match *e {
            Expr::Assign {
                ref name,
                ref value,
            } => {
                let val = self.evaluate(&*value)?;
                self.environment.assign(name.clone(), val)
            }
            Expr::Call {
                ref callee,
                ref paren,
                ref arguments,
            } => {
                let c = self.evaluate(&*callee)?;
                let mut args = Vec::new();
                for a in arguments {
                    let arg_val = self.evaluate(&*a)?;
                    args.push(arg_val);
                }

                match into_callable(c) {
                    Some(f) => f.call(self, args),
                    None => Err(Error::RuntimeError {
                        token: paren.clone(),
                        message: format!("Expression is not callable"),
                    }),
                }
            }
            Expr::Literal { ref value } => Ok(value.clone()),
            Expr::Grouping { ref expression } => self.evaluate(&*expression),
            Expr::Unary {
                ref right,
                ref operator,
            } => {
                let r_val = self.evaluate(&*right)?;
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
                ref left,
                ref right,
                ref operator,
            } => {
                let l_val = self.evaluate(&*left)?;
                let r_val = self.evaluate(&*right)?;
                check_number_operands(operator, &l_val, &r_val)?;

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
                    (_, a, b) => Err(Error::RuntimeError {
                        token: operator.clone(),
                        message: format!(
                            "There was some problem applying {:?} to operands {:?} and {:?}",
                            operator, a, b
                        ),
                    }),
                }
            }

            Expr::Logical {
                ref left,
                ref right,
                ref operator,
            } => {
                let l = self.evaluate(&*left)?;
                if operator.token_type == TokenType::Or && is_truthy(&l) {
                    return Ok(l);
                }

                if !is_truthy(&l) {
                    return Ok(l);
                }

                self.evaluate(&*right)
            }

            Expr::Variable { ref name } => self.environment.get(name),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RuntimeError {
                ref token,
                ref message,
            } => write!(f, "Runtime error at {:?}: {}", token, message),

            Error::Return(ref v) => write!(f, "Return {}", v),
        }
    }
}

fn check_number_operands(t: &Token, a: &LoxValue, b: &LoxValue) -> IResult<()> {
    match t.token_type {
        TokenType::Minus
        | TokenType::Slash
        | TokenType::Star
        | TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual => match (a, b) {
            (&LoxValue::Number(_), &LoxValue::Number(_)) => Ok(()),
            (_, _) => Err(Error::RuntimeError {
                token: t.clone(),
                message: format!("Operands {:?} and {:?} must both be numbers.", a, b),
            }),
        },
        _ => Ok(()),
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

fn into_callable(e: LoxValue) -> Option<Rc<LoxCallable>> {
    match e {
        LoxValue::Fn(f) => Some(f),
        _ => None,
    }
}
