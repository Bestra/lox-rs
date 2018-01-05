use token::{LoxValue, Token, TokenType};
use ast::{Expr, Program, Statement};

#[derive(Debug)]
pub struct ParseError {
    token: Token,
    message: String,
}

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(Program { statements })
    }

    fn declaration(&mut self) -> Result<Statement, ParseError> {
        if self.match_token(vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Statement, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?
            .clone();

        let initializer = if self.match_token(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Statement::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        if self.match_token(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.match_token(vec![TokenType::LeftBrace]) {
            Ok(Statement::Block {
                statements: self.block()?,
            })
        } else if self.match_token(vec![TokenType::If]) {
            self.if_statement()
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after statement.")?;
        Ok(Statement::Print { expression })
    }

    fn if_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' before 'if'.");
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'if'.");

        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;
        if self.match_token(vec![TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?))
        }

        Ok(Statement::If { condition, then_branch, else_branch })
    }

    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after statement.")?;
        Ok(Statement::Expression { expression })
    }

    fn expression(&mut self) -> Result<Box<Expr>, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, ParseError> {
        let expr = self.equality()?;
        if self.match_token(vec![TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            match *expr {
                Expr::Variable { name, .. } => Ok(Box::new(Expr::Assign { name, value: value })),
                _ => Err(ParseError {
                    token: equals,
                    message: "Invalid assignment target".to_string(),
                }),
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_token(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.addition()?;
        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.addition()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.multiplication()?;
        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.multiplication()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.unary()?;
        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Box::new(Expr::Unary {
                operator: operator,
                right: right,
            }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.match_token(vec![TokenType::True]) {
            return Ok(Box::new(Expr::Literal {
                value: LoxValue::Bool(true),
            }));
        }
        if self.match_token(vec![TokenType::False]) {
            return Ok(Box::new(Expr::Literal {
                value: LoxValue::Bool(false),
            }));
        }
        if self.match_token(vec![TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal {
                value: LoxValue::Nil,
            }));
        }
        if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Ok(Box::new(Expr::Literal {
                value: self.previous().clone().literal,
            }));
        }
        if self.match_token(vec![TokenType::LeftParen]) {
            let expression = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Box::new(Expr::Grouping { expression }));
        }

        if self.match_token(vec![TokenType::Identifier]) {
            let name = self.previous().clone();
            return Ok(Box::new(Expr::Variable { name }));
        }

        Err(ParseError {
            token: self.previous().to_owned(),
            message: "No matching primary".to_string(),
        })
    }

    fn consume(&mut self, t: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&t) {
            Ok(self.advance())
        } else {
            Err(ParseError {
                token: self.previous().to_owned(),
                message: message.to_string(),
            })
        }
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for t in token_types {
            if self.check(&t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, t: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *t
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn rewind(&mut self) {
        self.current -= 1;
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
