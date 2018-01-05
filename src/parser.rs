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
        if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
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
            self.print_statement()
        } else if self.match_token(&[TokenType::LeftBrace]) {
            Ok(Statement::Block {
                statements: self.block()?,
            })
        } else if self.match_token(&[TokenType::If]) {
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
            self.primary()
        }
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
