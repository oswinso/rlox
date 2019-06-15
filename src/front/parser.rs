use crate::front::ast_printer::AstPrinter;
use crate::front::expr::*;
use crate::front::stmt::{Declaration, Stmt};
use crate::front::token::Token;
use crate::front::token_type::TokenType;
use crate::report;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            self.declaration()
                .map(|declaration| statements.push(declaration));
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let res = if self.match_tokens(vec![TokenType::Let]) {
            self.variable_declaration()
        } else {
            self.statement()
        };
        if res.is_none() {
            self.synchronize();
        }
        res
    }

    fn variable_declaration(&mut self) -> Option<Stmt> {
        let name = self
            .consume(TokenType::Identifier("".into()), "Expected variable name")
            .unwrap()
            .clone();

        let initializer = if self.match_tokens(vec![TokenType::Equal]) {
            self.expression()
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        );
        Some(Stmt::Declaration(Declaration { name, initializer }))
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.match_tokens(vec![TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.");
        Some(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.");
        Some(Stmt::Expression(value))
    }

    fn expression(&mut self) -> Option<Expr> {
        self.ternary()
    }

    fn ternary(&mut self) -> Option<Expr> {
        let condition = self.equality()?;

        if self.match_tokens(vec![TokenType::QuestionMark]) {
            let question_mark: Token = self.previous().clone();
            let true_branch = self.equality()?;

            if self.match_tokens(vec![TokenType::Colon]) {
                let colon: Token = self.previous().clone();
                let false_branch = self.ternary()?;
                return Some(Expr::Ternary(Ternary::new(
                    condition,
                    true_branch,
                    false_branch,
                )));
            }
        }
        Some(condition)
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison()?;

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(expr, operator, right))
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
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        Some(expr)
    }

    fn addition(&mut self) -> Option<Expr> {
        let mut expr = self.multiplication()?;

        while self.match_tokens(vec![TokenType::Plus, TokenType::Minus]) {
            let operator: Token = self.previous().clone();
            let right = self.multiplication()?;
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        Some(expr)
    }

    fn multiplication(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;

        while self.match_tokens(vec![TokenType::Star, TokenType::Slash]) {
            let operator: Token = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            return Some(Expr::Unary(Unary::new(operator, right)));
        }
        let prim = self.primary()?;
        Some(prim)
    }

    fn primary(&mut self) -> Option<Expr> {
        let peeked = self.peek().clone().token_type;
        self.advance();
        match peeked {
            TokenType::False => Some(Expr::Literal(false.into())),
            TokenType::True => Some(Expr::Literal(true.into())),
            TokenType::Nil => Some(Expr::Literal(Literal::Nil.into())),
            TokenType::Number(num) => Some(Expr::Literal(num.into())),
            TokenType::String(string) => Some(Expr::Literal(string.into())),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression.");
                Some(Expr::Grouping(Grouping::new(expr)))
            }
            TokenType::Identifier(string) => Some(Expr::Variable(Variable {
                name: self.previous().clone(),
            })),
            c => {
                println!("Token: {:?}", c);
                panic!("lol")
            }
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
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(&token)
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
