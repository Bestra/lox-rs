#[derive(Debug,PartialEq)]
pub enum TokenType {
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
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,
    String,
    Number,
    Identifier,
    Unexpected,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

pub fn get_keyword(s: &str) -> TokenType {
    match s {
        "and" =>    TokenType::And,
        "class" =>  TokenType::Class,
        "else" =>   TokenType::Else,
        "false" =>  TokenType::False,
        "for" =>    TokenType::For,
        "fun" =>    TokenType::Fun,
        "if" =>     TokenType::If,
        "nil" =>    TokenType::Nil,
        "or" =>     TokenType::Or,
        "print" =>  TokenType::Print,
        "return" => TokenType::Return,
        "super" =>  TokenType::Super,
        "this" =>   TokenType::This,
        "true" =>   TokenType::True,
        "var" =>    TokenType::Var,
        "while" =>  TokenType::While,
        _ => TokenType::Identifier,
    }
}

#[derive(Debug)]
pub enum TokenLiteral {
    String(String),
    Number(f64),
    None,
}

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub line: u64,
    pub literal: TokenLiteral,
    pub token_type: TokenType,
}
