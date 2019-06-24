use crate::compiler::{Keyword, Position, Source, Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'src> {
    source: Peekable<Chars<'src>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'src> Scanner<'src> {
    pub fn new(source: Source<'src>) -> Scanner<'src> {
        Scanner {
            source: source.source.chars().peekable(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        use TokenKind::*;

        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(EOF);
        }

        let token_kind = match self.advance() {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ',' => Comma,
            '.' => Dot,
            '-' => Minus,
            '+' => Plus,
            ';' => Semicolon,
            '*' => Star,
            '?' => QuestionMark,
            ':' => Colon,

            '!' => {
                if self.try_consume('=') {
                    BangEqual
                } else {
                    Bang
                }
            }
            '=' => {
                if self.try_consume('=') {
                    EqualEqual
                } else {
                    Equal
                }
            }
            '<' => {
                if self.try_consume('=') {
                    LessEqual
                } else {
                    Less
                }
            }
            '>' => {
                if self.try_consume('=') {
                    GreaterEqual
                } else {
                    Greater
                }
            }
            '"' => self.string(),
            c => {
                if c.is_digit(10) {
                    self.number()
                } else if c.is_ascii_alphanumeric() || c == '_' {
                    self.identifier(c)
                } else {
                    self.error_kind("Invalid character")
                }
            }
        };

        self.make_token(token_kind)
    }

    fn make_token(&self, token_kind: TokenKind) -> Token {
        Token::new(
            token_kind,
            Position::new(self.start, self.current, self.line),
        )
    }

    fn error_kind(&self, error_message: &str) -> TokenKind {
        TokenKind::Error(error_message.to_owned())
    }

    fn error_token(&self, error_message: &str) -> Token {
        self.make_token(self.error_kind(error_message))
    }

    fn is_at_end(&mut self) -> bool {
        return self.source.peek().is_none();
    }

    fn peek(&mut self) -> char {
        *self.source.peek().unwrap()
    }

    fn peek_next(&self) -> char {
        let mut it = self.source.clone();
        it.nth(2).unwrap()
    }

    fn advance(&mut self) -> char {
        let c = self.source.next().unwrap();
        self.current += 1;
        c
    }

    fn try_consume(&mut self, c: char) -> bool {
        if *self.source.peek().unwrap() == c {
            self.advance();
            true
        } else {
            false
        }
    }

    fn string(&mut self) -> TokenKind {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_kind("Unterminated String");
        }

        self.advance();
        TokenKind::String
    }

    fn number(&mut self) -> TokenKind {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Check for fractional
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance(); // Consume the .
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        TokenKind::Number
    }

    fn identifier(&mut self, mut c: char) -> TokenKind {
        let mut buffer = String::with_capacity(16);
        buffer.push(c);

        while (c.is_ascii_alphanumeric() || c == '_') && !self.is_at_end() {
            buffer.push(self.advance());
            c = self.peek();
        }

        let keyword = self.get_keyword(&buffer);

        keyword.map_or(TokenKind::Identifier, |keyword| TokenKind::Keyword(keyword))
    }

    fn get_keyword(&self, buffer: &str) -> Option<Keyword> {
        use Keyword::*;
        match buffer {
            "and" => Some(And),
            "class" => Some(Class),
            "else" => Some(Else),
            "false" => Some(False),
            "for" => Some(For),
            "fun" => Some(Fun),
            "if" => Some(If),
            "nil" => Some(Nil),
            "or" => Some(Or),
            "print" => Some(Print),
            "return" => Some(Return),
            "super" => Some(Super),
            "this" => Some(This),
            "true" => Some(True),
            "let" => Some(Let),
            "while" => Some(While),
            _ => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.try_consume('/') {
                        // // Comment
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else if self.try_consume('*') {
                        // /* Comment block
                        while self.peek() != '*'
                            && self.peek_next() != '/'
                            && !self.is_at_end()
                            && self.peek_next() != '\0'
                        {
                            self.advance();
                        }
                        self.advance();
                        self.advance();
                    } else {
                        return;
                    }
                }
                _ => return,
            };
        }
    }
}
