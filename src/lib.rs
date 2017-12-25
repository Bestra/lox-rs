use std::process;
use std::str::FromStr;
mod token;
mod ast;
use token::{Token,TokenType,TokenLiteral};


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
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: u64,
    pub current: u64,
    pub line: u64,
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
            literal: TokenLiteral::None,
            line: self.line,
        };
        self.tokens.push(end);
        self
    }

    fn is_at_end(&self) -> bool {
        self.current as usize >= self.source.len()
    }

    fn scan_token(&mut self) -> () {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, TokenLiteral::None),
            ')' => self.add_token(TokenType::RightParen, TokenLiteral::None),
            '{' => self.add_token(TokenType::LeftBrace, TokenLiteral::None),
            '}' => self.add_token(TokenType::RightBrace, TokenLiteral::None),
            ',' => self.add_token(TokenType::Comma, TokenLiteral::None),
            '.' => self.add_token(TokenType::Dot, TokenLiteral::None),
            '-' => self.add_token(TokenType::Minus, TokenLiteral::None),
            '+' => self.add_token(TokenType::Plus, TokenLiteral::None),
            ';' => self.add_token(TokenType::Semicolon, TokenLiteral::None),
            '*' => self.add_token(TokenType::Star, TokenLiteral::None),
            '!' => {
                let t = if self.match_token('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t, TokenLiteral::None)
            }
            '=' => {
                let t = if self.match_token('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t, TokenLiteral::None)
            }
            '<' => {
                let t = if self.match_token('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t, TokenLiteral::None)
            }
            '>' => {
                let t = if self.match_token('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t, TokenLiteral::None)
            }

            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, TokenLiteral::None)
                }
            }
            '"' => self.string(),

            ' ' | '\r' | '\t' => (),
            '\n' => self.line = self.line + 1,
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => {
                while is_alphanumeric(self.peek()) {
                    self.advance();
                }

                let text = &self.current_substring();

                self.add_token(token::get_keyword(text), TokenLiteral::None)
            }
            _ => self.add_token(TokenType::Unexpected, TokenLiteral::None),
        }
    }

    fn current_substring(&self)  -> String {
        self.source.clone()[(self.start as usize)..(self.current as usize)].to_string()
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

        let value = &self.source.clone()[(self.start as usize)..(self.current as usize)];
        let num = f64::from_str(value).unwrap();
        self.add_token(TokenType::Number, TokenLiteral::Number(num))
    }


    fn string(&mut self) -> () {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Scanner error: unterminated string");
            process::exit(1);
        }

        self.advance(); // get the closing '"'
        let start = self.start as usize + 1;
        let current = self.current as usize - 1;

        let value = &self.source.clone()[start..current];
        self.add_token(TokenType::String, TokenLiteral::String(value.to_string()));
    }

    fn add_token(&mut self, t: TokenType, l: TokenLiteral) -> () {
        let start = self.start as usize;
        let current = self.current as usize;
        let lexeme = &self.source[start..current];
        let token = Token {
            token_type: t,
            lexeme: lexeme.to_string(),
            literal: l,
            line: self.line,
        };
        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current as usize).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        let pos = self.current as usize + 1;
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
        let current_char = self.source.chars().nth(self.current as usize).unwrap();
        if current_char != expected {
            return false;
        }

        self.current = self.current + 1;
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
