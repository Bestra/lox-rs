use std::collections::HashMap;
use token::{LoxValue, Token};
use interpreter::Error;

type Bindings = HashMap<String, LoxValue>;

#[derive(Debug, Clone)]
pub struct Environment {
    stack: Vec<Bindings>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            stack: vec![HashMap::new()],
        }
    }

    pub fn push(&mut self) {
        self.stack.push(HashMap::new())
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    fn find_frame_for_var(&mut self, name: &str) -> Option<&mut Bindings> {
        for frame in self.stack.iter_mut().rev() {
            if frame.contains_key(name) {
                return Some(frame);
            }
        }
        None
    }

    pub fn define(&mut self, name: String, value: LoxValue) -> () {
        let values = self.stack.last_mut().unwrap();
        values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<LoxValue, Error> {
        let lexeme = name.lexeme.clone();

        match self.find_frame_for_var(&lexeme) {
            Some(stack_frame) => {
                Ok(stack_frame.get(&lexeme).unwrap().to_owned())
            }
            None => Err(Error::RuntimeError {
                token: name.clone(),
                message: format!("Undefined variable {}.", lexeme),
            }),
        }
    }

    pub fn assign(&mut self, name: Token, value: LoxValue) -> Result<LoxValue, Error> {
        let lexeme = name.lexeme.clone();
        match self.find_frame_for_var(&lexeme) {
            Some(stack_frame) => {
                stack_frame.insert(lexeme, value.to_owned());
                Ok(value)
            }
            None => Err(Error::RuntimeError {
                token: name.clone(),
                message: format!("Undefined variable {}.", lexeme),
            }),
        }
    }
}
