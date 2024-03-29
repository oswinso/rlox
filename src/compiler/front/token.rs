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
        if let (TokenKind::Keyword(l), TokenKind::Keyword(r)) = (self, other) {
            l == r
        } else {
            std::mem::discriminant(self) == std::mem::discriminant(other)
        }
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

#[derive(Ord, Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<u32> for Precedence {
    fn from(num: u32) -> Self {
        use Precedence::*;
        match num {
            0 => None,
            1 => Assignment,
            2 => Or,
            3 => And,
            4 => Equality,
            5 => Comparison,
            6 => Term,
            7 => Factor,
            8 => Unary,
            9 => Call,
            10 => Primary,
            _ => unreachable!(),
        }
    }
}
