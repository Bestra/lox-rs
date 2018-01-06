use ast::{Expr, Program, Statement};
use std::fmt;
use std::mem::replace;
use token::{LoxValue, Token, TokenType};
use std::collections::HashMap;

pub struct RuntimeError {
    token: Token,
    message: String,
}

type IResult<T> = Result<T, RuntimeError>;

#[derive(Clone)]
struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    fn new(enclosing: Option<Box<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    fn define(&mut self, name: String, value: LoxValue) -> () {
        self.values.insert(name, value);
    }

    fn get(&mut self, name: &Token) -> IResult<LoxValue> {
        let lexeme = name.lexeme.clone();
        match self.values.get(&lexeme) {
            Some(v) => Ok(v.to_owned()),
            None => match self.enclosing {
                Some(ref mut e) => e.get(name),
                None => Err(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable {}.", lexeme),
                }),
            },
        }
    }

    fn assign(&mut self, name: Token, value: LoxValue) -> IResult<LoxValue> {
        let lexeme = name.lexeme.clone();
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value.to_owned());
            Ok(value)
        } else {
            match self.enclosing {
                Some(ref mut e) => e.assign(name, value),
                None => Err(RuntimeError {
                    token: name,
                    message: format!("Undefined variable {}.", lexeme),
                }),
            }
        }
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(None),
        }
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
                let parent = self.environment.clone();
                self.execute_block(statements, Environment::new(Some(Box::new(parent))))
            }
            Statement::Expression { ref expression } => {
                self.evaluate(&*expression)?;
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

    fn execute_block(&mut self, statements: &[Statement], env: Environment) -> IResult<()> {
        let previous_env = replace(&mut self.environment, env);
        for s in statements {
            match self.execute(s) {
                Ok(_r) => (),
                Err(r) => {
                    replace(&mut self.environment, previous_env);
                    return Err(r);
                }
            }
        }

        let used_env = replace(&mut self.environment, previous_env);
        self.environment = *(used_env.enclosing.unwrap()).clone();
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
                    (_, a, b) => Err(RuntimeError {
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

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime error at {:?}: {}", self.token, self.message)
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
            (_, _) => Err(RuntimeError {
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
