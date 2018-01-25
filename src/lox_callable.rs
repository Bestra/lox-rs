use token::LoxValue;
use std::fmt;
use interpreter::{Environment, Interpreter, RuntimeError};
use std::time::{SystemTime, UNIX_EPOCH};
use ast::{FunctionDeclaration};


pub trait LoxCallable: fmt::Debug {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError>;
    fn arity(&self) -> usize;
    fn name(&self) -> &str;
}

impl fmt::Display for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.name())
    }
}


#[derive(Debug)]
pub struct Clock;
impl LoxCallable for Clock {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError> {
        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Ok(LoxValue::Number(t as f64))
    }

    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> &str {
        "clock"
    }
}


#[derive(Debug)]
pub struct LoxFunction {
  declaration: FunctionDeclaration,
}

impl LoxFunction {
    pub fn new(declaration: FunctionDeclaration) -> LoxFunction {
        LoxFunction { declaration }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError> {
        let mut env = Environment::new(Some(Box::new(interpreter.globals.clone())));
        for (p, arg) in self.declaration.parameters.iter().zip(arguments.into_iter()) {
            env.define(p.lexeme.clone(), arg);
        }

        interpreter.execute_block(&self.declaration.body, env.clone())?;
        Ok(LoxValue::Nil)
    }
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }
    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }
}
