use std::time::{SystemTime, UNIX_EPOCH};
use ::interpreter::{Error, Interpreter};
use ::lox_value::LoxValue;
use ::lox_callable::LoxCallable;
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
