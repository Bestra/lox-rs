use lox_value::LoxValue;
use std::fmt;
use interpreter::{Error, Interpreter};
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
