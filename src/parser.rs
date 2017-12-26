use token::{Token, LoxValue, TokenType};
use std::process;
use ast::Expr;
pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        *self.expression()
    }

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();
        while self.match_token(vec![TokenType::Bang, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }

        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr = self.addition();
        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.addition();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }

        expr
    }

    fn addition(&mut self) -> Box<Expr> {
        let mut expr = self.multiplication();
        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.multiplication();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }

        expr
    }

    fn multiplication(&mut self) -> Box<Expr> {
        let mut expr = self.unary();
        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            Box::new(Expr::Unary {
                operator: operator,
                right: right,
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.match_token(vec![TokenType::True]) {
            return Box::new(Expr::Literal {
                value: LoxValue::Bool(true),
            });
        }
        if self.match_token(vec![TokenType::False]) {
            return Box::new(Expr::Literal {
                value: LoxValue::Bool(false),
            });
        }
        if self.match_token(vec![TokenType::Nil]) {
            return Box::new(Expr::Literal {
                value: LoxValue::Nil,
            });
        }
        if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Box::new(Expr::Literal {
                value: self.previous().clone().literal,
            });
        }
        if self.match_token(vec![TokenType::LeftParen]) {
            let expression = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Box::new(Expr::Grouping { expression });
        }
        eprintln!("No matching primary");
        process::exit(1);
    }

    fn consume(&mut self, t: TokenType, message: &str) -> &Token {
        if self.check(t) {
            self.advance()
        } else {
            eprintln!("{}", message);
            process::exit(1);
        }
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, t: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == t
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
        self.tokens.iter().nth(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.iter().nth(self.current - 1).unwrap()
    }
}
