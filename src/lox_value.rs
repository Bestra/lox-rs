use lox_callable::LoxCallable;
use std::fmt;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub enum LoxValue {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
    Fn(Rc<LoxCallable>),
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &LoxValue) -> bool {
        match (self, other) {
            (&LoxValue::String(ref a), &LoxValue::String(ref b)) => a == b,
            (&LoxValue::Number(ref a), &LoxValue::Number(ref b)) => a == b,
            (&LoxValue::Bool(ref a), &LoxValue::Bool(ref b)) => a == b,
            (&LoxValue::Nil, &LoxValue::Nil) => true,
            (&LoxValue::Fn(ref a), &LoxValue::Fn(ref b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoxValue::String(ref s) => write!(f, "{}", s),
            LoxValue::Number(ref s) => write!(f, "{}", s),
            LoxValue::Bool(ref s) => write!(f, "{}", s),
            LoxValue::Nil => write!(f, "{}", "nil"),
            LoxValue::Fn(ref fun) => write!(f, "{}", fun),
        }
    }
}
