use ast::{Expr, FunctionDeclaration, Program, Statement};
use std::collections::HashMap;
use interpreter::Interpreter;
use token::Token;
use std::mem::replace;

#[derive(Debug, PartialEq)]
enum Status {
    Uninitialized,
    Initialized,
}

enum FunctionType {
    None,
    Fuction,
}

#[derive(Debug)]
pub struct Error {
    token: Token,
    message: String,
}

pub struct Resolver {
    scopes: Vec<HashMap<String, Status>>,
    current_function: FunctionType,
    pub interpreter: Interpreter,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Resolver {
        Resolver {
            scopes: vec![],
            interpreter: interpreter,
            current_function: FunctionType::None,
        }
    }

    pub fn resolve(&mut self, p: &Program) -> Result<(), Error> {
        self.begin_scope();
        for s in &p.statements {
            self.resolve_statement(&s)?;
        }
        self.end_scope();
        Ok(())
    }

    pub fn visit_statement(&mut self, s: &Statement) -> Result<(), Error>  {
        match *s {
            Statement::Block { ref statements } => {
                self.begin_scope();
                for s in statements {
                    self.resolve_statement(&s)?;
                }
                self.end_scope();
            }

            Statement::Var {
                ref name,
                ref initializer,
            } => {
                self.declare(name)?;
                if let &Some(ref e) = initializer {
                    self.resolve_expr(&e)?;
                }
                self.define(name);
            }

            Statement::Function(ref statement) => {
                self.declare(&statement.name)?;
                self.define(&statement.name);
                self.resolve_function(&statement, FunctionType::Fuction)?;
            }

            Statement::Expression { ref expression } => {
                self.resolve_expr(expression)?;
            }

            Statement::If {
                ref condition,
                ref then_branch,
                ref else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_statement(&*then_branch)?;
                if let &Some(ref s) = else_branch {
                    self.resolve_statement(&s)?;
                }
            }

            Statement::Print { ref expression } => {
                self.resolve_expr(expression)?;
            }

            Statement::Return {
                ref value,
                ref keyword,
            } => {
                match self.current_function {
                    FunctionType::None => {
                        return Err(Error {
                            token: keyword.clone(),
                            message: "Cannot return from top-level code."
                                .to_string(),
                        });
                    }
                    FunctionType::Fuction => ()
                };
                if let &Some(ref v) = value {
                    self.resolve_expr(&v)?;
                }
            }

            Statement::While {
                ref condition,
                ref body,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_statement(body)?;
            }
        }

        Ok(())
    }

    fn resolve_function(&mut self, function: &FunctionDeclaration, function_type: FunctionType) -> Result<(), Error> {

        let enclosing_type = replace(&mut self.current_function, function_type);
        self.begin_scope();
        for p in &function.parameters {
            self.declare(&p)?;
            self.define(&p);
        }

        for statement in &function.body {
            self.resolve_statement(&statement)?;
        }

        self.end_scope();
        self.current_function = enclosing_type;
        Ok(())
    }

    pub fn visit_expression(&mut self, e: &Expr) -> Result<(), Error> {
        match *e {
            Expr::Variable { ref name } => {
                if let Some(scope) = self.scopes.last_mut() {
                    if let Some(&Status::Uninitialized) = scope.get(&name.lexeme) {
                        return Err(Error {
                            token: name.clone(),
                            message: "Cannot read local variable in its own initializer."
                                .to_string(),
                        });
                    }
                }
                self.resolve_local(e, name);
                Ok(())
            }

            Expr::Assign {
                ref value,
                ref name,
            } => {
                self.resolve_expr(value)?;
                self.resolve_local(e, name);
                Ok(())
            }

            Expr::Binary {
                ref left,
                ref right,
                ..
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }

            Expr::Call {
                ref callee,
                ref arguments,
                ..
            } => {
                self.resolve_expr(callee)?;
                for a in arguments {
                    self.resolve_expr(a)?;
                }
                Ok(())
            }

            Expr::Grouping {
                ref expression,
                ..
            } => {
                self.resolve_expr(expression)?;
                Ok(())
            }

            Expr::Literal {
                ..
            } => Ok(()),

            Expr::Logical {
                ref left,
                ref right,
                ..
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }

            Expr::Unary {
                ref right,
                ..
            } => {
                self.resolve_expr(right)?;
                Ok(())
            }
        }
    }

    /*

   0 []
   1 []
   2 [foo]
   3 []

    */
    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for (i, scope) in self.scopes.iter_mut().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(&expr, i);
                return;
            }
        }
    }

    fn declare(&mut self, name: &Token) -> Result<(), Error> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                return Err(Error {
                    token: name.clone(),
                    message: "Variable with this name already defined in scope.".to_string(),
                });
            }
            scope.insert(name.lexeme.clone(), Status::Uninitialized);
        }

        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), Status::Initialized);
        }
    }

    fn resolve_statement(&mut self, s: &Statement) -> Result<(), Error> {
        self.visit_statement(s)
    }

    fn resolve_expr(&mut self, e: &Expr) -> Result<(), Error> {
        self.visit_expression(e)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}
