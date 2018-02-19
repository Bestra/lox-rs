use lox_value::LoxValue;
use interpreter::{Error, Interpreter};
use lox_callable::LoxCallable;
use ast::FunctionDeclaration;
use environment::Environment;
use std::mem::replace;

#[derive(Debug)]
pub struct LoxFunction {
    declaration: FunctionDeclaration,
    closure: Environment,
}

impl LoxFunction {
    pub fn new(declaration: FunctionDeclaration, env: &Environment) -> LoxFunction {
        let closure = Environment::from(env);
        LoxFunction {
            declaration,
            closure,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, Error> {
        // we need to evaluate the function in the context of its closure,
        // not whatever the interpreter's current environment is.
        let old_env = replace(
            &mut interpreter.environment,
            Environment::from(&self.closure),
        );

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

        // swap the environment back
        interpreter.environment = old_env;
        ret
    }
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }
    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }
}
