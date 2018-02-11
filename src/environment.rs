use std::collections::HashMap;
use token::{LoxValue, Token};
use interpreter::Error;
use std::cell::RefCell;
use std::rc::Rc;

type Bindings = Rc<RefCell<HashMap<String, LoxValue>>>;

#[derive(Debug, Clone)]
pub struct Environment {
    stack: Vec<Bindings>,
}

fn new_binding() -> Bindings {
    Rc::new(RefCell::new(HashMap::new()))
}

impl Environment {
    pub fn from(source_env: &Environment) -> Environment {
        Environment {
            stack: source_env.stack.iter().map(|e| Rc::clone(e)).collect(),
        }
    }

    pub fn new() -> Environment {
        Environment {
            stack: vec![new_binding()],
        }
    }

    pub fn push(&mut self) {
        self.stack.push(new_binding())
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    fn find_frame_for_var(&mut self, name: &str) -> Option<Bindings> {
        for frame in self.stack.iter().rev() {
            if frame.borrow_mut().contains_key(name) {
                return Some(Rc::clone(frame));
            }
        }
        None
    }

    pub fn define(&mut self, name: String, value: LoxValue) -> () {
        let values = self.stack.last().unwrap();

        values.borrow_mut().insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<LoxValue, Error> {
        let lexeme = name.lexeme.clone();

        match self.find_frame_for_var(&lexeme) {
            Some(stack_frame) => Ok(stack_frame.borrow_mut().get(&lexeme).unwrap().to_owned()),
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
                stack_frame.borrow_mut().insert(lexeme, value.to_owned());
                Ok(value)
            }
            None => Err(Error::RuntimeError {
                token: name.clone(),
                message: format!("Undefined variable {}.", lexeme),
            }),
        }
    }
}
