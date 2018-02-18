use std::fmt;
use lox_callable::LoxCallable;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
#[derive(Debug, PartialEq, Clone)]
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
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}

#[derive(Debug, Clone)]
pub enum LoxValue {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
    Fn(Rc<LoxCallable>),
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &LoxValue) -> bool {
        match (self, other) {
            (&LoxValue::String(ref a), &LoxValue::String(ref b)) => a == b,
            (&LoxValue::Number(ref a), &LoxValue::Number(ref b)) => a == b,
            (&LoxValue::Bool(ref a), &LoxValue::Bool(ref b)) => a == b,
            (&LoxValue::Nil, &LoxValue::Nil) => true,
            (&LoxValue::Fn(ref a), &LoxValue::Fn(ref b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoxValue::String(ref s) => write!(f, "{}", s),
            LoxValue::Number(ref s) => write!(f, "{}", s),
            LoxValue::Bool(ref s) => write!(f, "{}", s),
            LoxValue::Nil => write!(f, "{}", "nil"),
            LoxValue::Fn(ref fun) => write!(f, "{}", fun),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub line: usize,
    pub position: usize,
    pub literal: LoxValue,
    pub token_type: TokenType,
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.position == other.position
    }
}

impl Eq for Token {}
