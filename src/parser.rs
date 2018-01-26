use token::{LoxValue, Token, TokenType};
use ast::{Expr, FunctionDeclaration, Program, Statement};

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
        if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else if self.match_token(&[TokenType::Fun]) {
            self.function("function")
        } else {
            self.statement()
        }
    }

    fn function(&mut self, kind: &str) -> ParseResult<Statement> {
        let name = self.consume(TokenType::Identifier, "Expect function name.")?.clone();
        self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if params.len() >= 8 {
                    return Err(ParseError {
                        token: self.peek().clone(),
                        message: "Cannot have more than 8 parameters.".to_string()}
                    );
                }
                let p = self.consume(TokenType::Identifier, "Expect parameter name.")?.clone();
                params.push(p);
                if !self.match_token(&[TokenType::Comma]) {
                    break
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;
        let body = self.block()?;
        Ok(Statement::Function(FunctionDeclaration { name: name, body: body, parameters: params }))
    }

    fn var_declaration(&mut self) -> Result<Statement, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?
            .clone();

        let initializer = if self.match_token(&[TokenType::Equal]) {
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
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return Ok(Statement::Block {
                statements: self.block()?,
            });
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.match_token(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.match_token(&[TokenType::For]) {
            return self.for_statement();
        }

        self.expression_statement()
    }

    fn for_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.match_token(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let for_condition = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let for_increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after 'for' clauses.")?;

        let mut body = self.statement()?;
        if let Some(increment) = for_increment {
            body = Statement::Block {
                statements: vec![
                    body,
                    Statement::Expression {
                        expression: increment,
                    },
                ],
            };
        }

        match for_condition {
            None => {
                let loop_condition = Expr::Literal {
                    value: LoxValue::Bool(true),
                };
                body = Statement::While {
                    condition: Box::new(loop_condition),
                    body: Box::new(body),
                };
            }
            Some(c) => {
                body = Statement::While {
                    condition: c,
                    body: Box::new(body),
                };
            }
        }

        if let Some(i) = initializer {
            body = Statement::Block {
                statements: vec![i, body],
            };
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'while'.")?;
        let body = self.statement()?;

        Ok(Statement::While {
            condition,
            body: Box::new(body),
        })
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

    fn return_statement(&mut self) -> ParseResult<Statement> {
        let keyword = self.previous().clone();
        let value = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        return Ok(Statement::Return { keyword, value });
    }

    fn if_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' before 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'if'.")?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;
        if self.match_token(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?))
        }

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after statement.")?;
        Ok(Statement::Expression { expression })
    }

    fn expression(&mut self) -> ParseResult<Box<Expr>> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Box<Expr>> {
        let expr = self.or()?;
        if self.match_token(&[TokenType::Equal]) {
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

    /// This fn implements equality, comparison, addition, and multiplication
    fn binary_expr(
        &mut self,
        left: fn(&mut Self) -> ParseResult<Box<Expr>>,
        matches: &[TokenType],
        right: fn(&mut Self) -> ParseResult<Box<Expr>>,
    ) -> ParseResult<Box<Expr>> {
        let mut expr = left(self)?;
        while self.match_token(matches) {
            let operator = self.previous().clone();
            let right = right(self)?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn or(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.and()?;
        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Box::new(Expr::Logical {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn and(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.equality()?;
        while self.match_token(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Box::new(Expr::Logical {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Box<Expr>> {
        self.binary_expr(
            Parser::comparison,
            &[TokenType::EqualEqual, TokenType::BangEqual],
            Parser::comparison,
        )
    }

    fn comparison(&mut self) -> ParseResult<Box<Expr>> {
        self.binary_expr(
            Parser::addition,
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
            Parser::addition,
        )
    }

    fn addition(&mut self) -> ParseResult<Box<Expr>> {
        self.binary_expr(
            Parser::multiplication,
            &[TokenType::Minus, TokenType::Plus],
            Parser::multiplication,
        )
    }

    fn multiplication(&mut self) -> ParseResult<Box<Expr>> {
        self.binary_expr(
            Parser::unary,
            &[TokenType::Slash, TokenType::Star],
            Parser::unary,
        )
    }

    fn unary(&mut self) -> ParseResult<Box<Expr>> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Box::new(Expr::Unary {
                operator: operator,
                right: right,
            }))
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Box<Expr>) -> ParseResult<Box<Expr>>{
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 8 {
                   // TODO: This should not make the parser blow up
                   return Err(ParseError {
                        token: self.peek().clone(),
                        message: "Cannot have more than 8 arguments".to_string(),
                    });
                }
                let next_expr = self.expression()?;
                arguments.push(*next_expr);
                if !self.match_token(&[TokenType::Comma]) { break; }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?.clone();
        Ok(Box::new(Expr::Call { callee, paren, arguments }))
    }

    fn primary(&mut self) -> ParseResult<Box<Expr>> {
        match self.advance().token_type {
            TokenType::True => Ok(Expr::Literal {
                value: LoxValue::Bool(true),
            }),
            TokenType::False => Ok(Expr::Literal {
                value: LoxValue::Bool(false),
            }),
            TokenType::Nil => Ok(Expr::Literal {
                value: LoxValue::Nil,
            }),
            TokenType::Number | TokenType::String => Ok(Expr::Literal {
                value: self.previous().clone().literal,
            }),
            TokenType::LeftParen => {
                let expression = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping { expression })
            }

            TokenType::Identifier => {
                let name = self.previous().clone();
                Ok(Expr::Variable { name })
            }

            _ => Err(ParseError {
                token: self.previous().to_owned(),
                message: "No matching primary".to_string(),
            }),
        }.and_then(|e| Ok(Box::new(e)))
    }

    fn consume(&mut self, t: TokenType, message: &str) -> ParseResult<&Token> {
        if self.check(&t) {
            Ok(self.advance())
        } else {
            Err(ParseError {
                token: self.previous().to_owned(),
                message: message.to_string(),
            })
        }
    }

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for t in token_types {
            if self.check(t) {
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
