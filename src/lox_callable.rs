use token::LoxValue;
use std::fmt;
use interpreter::{Error, Interpreter};
use std::time::{SystemTime, UNIX_EPOCH};
use ast::FunctionDeclaration;

pub trait LoxCallable: fmt::Debug {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, Error>;
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
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, Error> {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
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
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, Error> {
        interpreter.environment.push();
        for (p, arg) in self.declaration
            .parameters
            .iter()
            .zip(arguments.into_iter())
        {
            interpreter.environment.define(p.lexeme.clone(), arg);
        }

        let ret = match interpreter.execute_block(&self.declaration.body) {
            Ok(_) => Ok(LoxValue::Nil),
            Err(Error::Return(v)) => Ok(v),
            Err(e) => Err(e),
        };
        interpreter.environment.pop();
        ret
    }
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }
    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }
}
