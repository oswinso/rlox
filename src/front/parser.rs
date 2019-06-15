use crate::front::expr::*;
use crate::front::token::Token;
use crate::front::token_type::TokenType;
use crate::report;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison()?;

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: Box::new(operator),
                right: Box::new(right),
            })
        }
        Some(expr)
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.addition()?;

        while self.match_tokens(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous().clone();
            let right = self.addition()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: Box::new(operator),
                right: Box::new(right),
            })
        }
        Some(expr)
    }

    fn addition(&mut self) -> Option<Expr> {
        let mut expr = self.multiplication()?;

        while self.match_tokens(vec![TokenType::Plus, TokenType::Minus]) {
            let operator: Token = self.previous().clone();
            let right = self.multiplication()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: Box::new(operator),
                right: Box::new(right),
            })
        }
        Some(expr)
    }

    fn multiplication(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;

        while self.match_tokens(vec![TokenType::Star, TokenType::Slash]) {
            let operator: Token = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: Box::new(operator),
                right: Box::new(right),
            })
        }
        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            return Some(Expr::Unary(Unary {
                operator: Box::new(operator),
                right: Box::new(right),
            }));
        }
        let prim = self.primary()?;
        Some(prim)
    }

    fn primary(&mut self) -> Option<Expr> {
        let peeked = self.peek().clone().token_type;
        self.advance();
        match peeked {
            TokenType::False => Some(Expr::Literal(Literal::Bool(false))),
            TokenType::True => Some(Expr::Literal(Literal::Bool(true))),
            TokenType::Nil => Some(Expr::Literal(Literal::Nil)),
            TokenType::Number(num) => Some(Expr::Literal(Literal::Number(num))),
            TokenType::String(string) => Some(Expr::Literal(Literal::String(string))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression.");
                Some(Expr::Grouping(Grouping {
                    expression: Box::new(expr),
                }))
            }
            c => {
                println!("Token: {:?}", c);
                panic!("lol")
            },
        }
    }

    fn match_tokens(&mut self, tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, error_message: &str) -> Option<&Token> {
        if self.check(token_type) {
            return Some(self.advance());
        }
        let next = self.peek();
        self.error(next, error_message);
        None
    }

    fn error(&self, token: &Token, error_message: &str) {
        match token.token_type {
            TokenType::Eof => report(token.line, " at end", error_message),
            _ => report(
                token.line,
                &format!(" at '{}'", token.lexeme),
                error_message,
            ),
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }
}
