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

#[derive(Debug, Clone)]
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

impl TokenKind {
    pub fn error_type() -> TokenKind {
        TokenKind::Error("".into())
    }

    pub fn try_into_error(&self) -> Option<&str> {
        if let TokenKind::Error(message) = &self {
            Some(message)
        } else {
            None
        }
    }
}

impl PartialEq for TokenKind {
    fn eq(&self, other: &TokenKind) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
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
