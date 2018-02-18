use std::process;
use std::str::FromStr;
use token::{LoxValue, Token, TokenType};

fn is_digit(c: char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false,
    }
}

fn is_alpha(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '_' => true,
        _ => false,
    }
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> &mut Scanner {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let end = Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: LoxValue::Nil,
            position: self.start,
            line: self.line,
        };
        self.tokens.push(end);
        self
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> () {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, LoxValue::Nil),
            ')' => self.add_token(TokenType::RightParen, LoxValue::Nil),
            '{' => self.add_token(TokenType::LeftBrace, LoxValue::Nil),
            '}' => self.add_token(TokenType::RightBrace, LoxValue::Nil),
            ',' => self.add_token(TokenType::Comma, LoxValue::Nil),
            '.' => self.add_token(TokenType::Dot, LoxValue::Nil),
            '-' => self.add_token(TokenType::Minus, LoxValue::Nil),
            '+' => self.add_token(TokenType::Plus, LoxValue::Nil),
            ';' => self.add_token(TokenType::Semicolon, LoxValue::Nil),
            '*' => self.add_token(TokenType::Star, LoxValue::Nil),
            '!' => {
                let t = if self.match_token('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t, LoxValue::Nil)
            }
            '=' => {
                let t = if self.match_token('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t, LoxValue::Nil)
            }
            '<' => {
                let t = if self.match_token('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t, LoxValue::Nil)
            }
            '>' => {
                let t = if self.match_token('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t, LoxValue::Nil)
            }

            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, LoxValue::Nil)
                }
            }
            '"' => self.string(),

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => {
                while is_alphanumeric(self.peek()) {
                    self.advance();
                }

                let text = &self.current_substring();

                self.add_token(super::token::get_keyword(text), LoxValue::Nil)
            }
            _ => self.add_token(TokenType::Unexpected, LoxValue::Nil),
        }
    }

    fn current_substring(&self) -> String {
        self.source.clone()[(self.start)..(self.current)].to_string()
    }

    fn number(&mut self) -> () {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            // consume the "."
            self.advance();
        }

        while is_digit(self.peek()) {
            self.advance();
        }

        let value = &self.source.clone()[(self.start)..(self.current)];
        let num = f64::from_str(value).unwrap();
        self.add_token(TokenType::Number, LoxValue::Number(num))
    }

    fn string(&mut self) -> () {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Scanner error: unterminated string");
            process::exit(1);
        }

        self.advance(); // get the closing '"'
        let start = self.start + 1;
        let current = self.current - 1;

        let value = &self.source.clone()[start..current];
        self.add_token(TokenType::String, LoxValue::String(value.to_string()));
    }

    fn add_token(&mut self, t: TokenType, l: LoxValue) -> () {
        let start = self.start;
        let current = self.current;
        let lexeme = &self.source[start..current];
        let token = Token {
            token_type: t,
            position: self.start,
            lexeme: lexeme.to_string(),
            literal: l,
            line: self.line,
        };
        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        let pos = self.current + 1;
        if pos >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(pos).unwrap()
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let current_char = self.source.chars().nth(self.current).unwrap();
        if current_char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use Scanner;
    use TokenType;
    fn check_token_type(s: &str, tt: TokenType) {
        let mut scanner = Scanner::new(s.to_string());
        scanner.scan_tokens();
        let t = scanner.tokens.iter().nth(0).unwrap();
        assert_eq!(t.token_type, tt);
    }
    #[test]
    fn parses_number() {
        check_token_type("12.5", TokenType::Number)
    }
    #[test]
    fn parses_string() {
        check_token_type("\"cool\"", TokenType::String)
    }
}
