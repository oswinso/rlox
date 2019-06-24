use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenKind,
    pub position: Position,
}

impl Token {
    pub fn new(ty: TokenKind, position: Position) -> Token {
        Token { ty, position }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl Position {
    pub fn new(start: usize, end: usize, line: usize) -> Position {
        Position { start, end, line }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // One character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    QuestionMark,
    Colon,

    // One or two characters
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    Keyword(Keyword),

    Error(String),

    EOF,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Let,
    While,
}
