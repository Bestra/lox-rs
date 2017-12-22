extern crate rprompt;

#[derive(Debug)]
enum TokenType {
    Eof,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
}

fn token_type_from_char(c: char) -> TokenType {
    match c {
        '(' => TokenType::LeftParen,
        ')' => TokenType::RightParen,
        '{' => TokenType::LeftBrace,
        '}' => TokenType::RightBrace,
        ',' => TokenType::Comma,
        '.' => TokenType::Dot,
        '-' => TokenType::Minus,
        '+' => TokenType::Plus,
        ';' => TokenType::Semicolon,
        '*' => TokenType::Star,
        _ => TokenType::Eof,
    }
}

#[derive(Debug)]
struct Token {
    lexeme: String,
    line: u64,
    literal: String,
    token_type: TokenType,
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    fn scan_tokens(&mut self) -> &mut Scanner {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let end = Token {token_type: TokenType::Eof, lexeme: "".to_string(), literal: "".to_string(), line: self.line};
        self.tokens.push(end);
        self
    }

    fn is_at_end(&self) -> bool {
        self.current as usize >= self.source.len()
    }

    fn scan_token(&mut self) -> () {
        let c = self.advance();
        self.add_token(token_type_from_char(c))
    }

    fn add_token(&mut self, t: TokenType) -> () {
        let start = self.start as usize;
        let current = self.current as usize;
        let lexeme = &self.source[start..current];
        let token = Token {token_type: t, lexeme: lexeme.to_string(), literal: "".to_string(), line: self.line};
        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        self.source.chars().nth(self.current as usize - 1).unwrap()
    }
}

fn main() {
    let input = rprompt::prompt_reply_stdout(">").unwrap();

    let mut scanner = Scanner {
        source: input,
        tokens: vec![],
        start: 0,
        current: 0,
        line: 1
    };

    scanner.scan_tokens();
    println!("{:?}", scanner.tokens);
}
