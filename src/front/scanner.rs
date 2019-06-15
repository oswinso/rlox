use crate::error;
use crate::front::token::Token;
use crate::front::token_type::TokenType;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let token = Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line: 0,
        };
        self.tokens.push(token);
        std::mem::replace(&mut self.tokens, Vec::new())
    }

    fn scan_token(&mut self) {
        let c = self.consume().unwrap();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '?' => self.add_token(TokenType::QuestionMark),
            ':' => self.add_token(TokenType::Colon),

            '!' => {
                if self.try_consume('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.try_consume('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.try_consume('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.try_consume('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.try_consume('/') {
                    // // Comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.consume();
                    }
                } else if self.try_consume('*') {
                    // /* Comment block
                    while self.peek() != '*'
                        && self.peek_next() != '/'
                        && !self.is_at_end()
                        && self.peek_next() != '\0'
                    {
                        self.consume();
                    }
                    self.consume();
                    self.consume();
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.handle_string(),
            c => {
                if c.is_digit(10) {
                    self.handle_number();
                } else if c.is_ascii_alphanumeric() || c == '_' {
                    self.handle_identifier();
                } else {
                    error(self.line, &format!("Unexpected token {}", c))
                }
            }
        }
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.consume();
        }

        // Unterminated string
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
        }

        // Closing "
        self.consume();

        // Remove surrounding quotes
        let literal: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take((self.current - self.start) - 2)
            .collect();
        self.add_token(TokenType::String(literal))
    }

    fn handle_number(&mut self) {
        while self.peek().is_digit(10) {
            self.consume();
        }

        // Check for fractional
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.consume(); // Consume the .
            while self.peek().is_digit(10) {
                self.consume();
            }
        }

        let literal: f64 = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>()
            .parse()
            .unwrap();
        self.add_token(TokenType::Number(literal));
    }

    fn handle_identifier(&mut self) {
        let mut c = self.peek();
        while (c.is_ascii_alphanumeric() || c == '_') && !self.is_at_end() {
            self.consume();
            c = self.peek();
        }
        let identifier = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>();

        self.add_token(
            self.get_keyword(&identifier)
                .unwrap_or(TokenType::Identifier(identifier)),
        )
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token {
            token_type,
            lexeme,
            line: self.line,
        })
    }

    fn consume(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn try_consume(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&mut self) -> char {
        return self.source.chars().nth(self.current + 1).unwrap_or('\0');
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn get_keyword(&self, string: &str) -> Option<TokenType> {
        match string {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "let" => Some(TokenType::Let),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}
