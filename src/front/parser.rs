use crate::front::expr::*;
use crate::front::stmt::{Block, ClassDecl, Declaration, FunctionDecl, If, Return, Stmt, While};
use crate::front::token::Token;
use crate::front::token_type::TokenType;
use crate::{error, report};

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
        } else if self.match_tokens(vec![TokenType::Fun]) {
            self.function("function")
        } else if self.match_tokens(vec![TokenType::Class]) {
            self.class_declaration()
        } else {
            self.statement()
        };
        if res.is_none() {
            self.synchronize();
        }
        res
    }

    fn class_declaration(&mut self) -> Option<Stmt> {
        let name = self
            .consume(TokenType::Identifier("".into()), "Expected class name")?
            .clone();
        self.consume(TokenType::LeftBrace, "Expected '{' before class body");

        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Stmt::Function(function_decl) = self.function("method")? {
                methods.push(function_decl);
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after class body");
        Some(Stmt::Class(ClassDecl::new(name, methods)))
    }

    fn function(&mut self, kind: &str) -> Option<Stmt> {
        let name = self
            .consume(
                TokenType::Identifier("".into()),
                &format!("Expected {} name", kind),
            )?
            .clone();
        let mut parameters = Vec::new();
        self.consume(
            TokenType::LeftParen,
            "Expected '(' after function declaration",
        );
        if !self.check(TokenType::RightParen) {
            while {
                if parameters.len() >= 8 {
                    error(self.peek().line, "Cannot have more than 8 parameters");
                }

                parameters.push(
                    self.consume(TokenType::Identifier("".into()), "Expected parameter name")?
                        .clone(),
                );
                self.match_tokens(vec![TokenType::Comma])
            } {}
        }
        self.consume(TokenType::RightParen, "Expected ')' after parameters");

        self.consume(
            TokenType::LeftBrace,
            &format!("Expected '{{' before {} body.", kind),
        );

        let body = self.block();

        Some(Stmt::Function(FunctionDecl::new(&name, parameters, body)))
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
        } else if self.match_tokens(vec![TokenType::LeftBrace]) {
            Some(Stmt::Block(Block::new(self.block())))
        } else if self.match_tokens(vec![TokenType::If]) {
            self.if_statement()
        } else if self.match_tokens(vec![TokenType::While]) {
            self.while_statement()
        } else if self.match_tokens(vec![TokenType::For]) {
            self.for_statement()
        } else if self.match_tokens(vec![TokenType::Return]) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self) -> Option<Stmt> {
        let keyword = self.previous().clone();
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expected ';' after return");
        Some(Stmt::Return(Return::new(keyword, value)))
    }

    fn for_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'.");

        let initializer = if self.match_tokens(vec![TokenType::Semicolon]) {
            None
        } else if self.match_tokens(vec![TokenType::Let]) {
            self.variable_declaration()
        } else {
            self.expression_statement()
        };

        let condition = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal(true.into())
        };
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition.");

        let increment = if !self.check(TokenType::RightParen) {
            self.expression()
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expected ')' after loop increment.");

        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Stmt::Block(Block::new(vec![body, Stmt::Expression(increment)]));
        };

        body = Stmt::While(While::new(condition, body));

        if let Some(initializer) = initializer {
            body = Stmt::Block(Block::new(vec![initializer, body]));
        }

        Some(body)
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'.");
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expected ')' after 'while' condition.",
        );
        let body = self.statement()?;

        Some(Stmt::While(While::new(condition, body)))
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'.");
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after 'if' condition.");

        let then_branch = self.statement()?;
        let else_branch = if self.match_tokens(vec![TokenType::Else]) {
            self.statement()
        } else {
            None
        };

        Some(Stmt::If(If::new(condition, then_branch, else_branch)))
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            self.declaration().map(|decl| statements.push(decl));
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block");
        statements
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
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Expr> {
        let expr = self.ternary()?;

        if self.match_tokens(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(var) = expr {
                return Some(Expr::Assign(Assign::new(var.name, value)));
            } else if let Expr::Get(get) = expr {
                return Some(Expr::Set(Set::new(*get.object, *get.name, value)));
            }
        }
        Some(expr)
    }

    fn ternary(&mut self) -> Option<Expr> {
        let condition = self.or()?;

        if self.match_tokens(vec![TokenType::QuestionMark]) {
            let question_mark: Token = self.previous().clone();
            let true_branch = self.or()?;

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

    fn or(&mut self) -> Option<Expr> {
        let mut expr = self.and()?;

        while self.match_tokens(vec![TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Binary::new(expr, operator, right));
        }
        Some(expr)
    }

    fn and(&mut self) -> Option<Expr> {
        let mut expr = self.equality()?;

        while self.match_tokens(vec![TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Binary::new(expr, operator, right));
        }
        Some(expr)
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
        let prim = self.call()?;
        Some(prim)
    }

    fn call(&mut self) -> Option<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_tokens(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_tokens(vec![TokenType::Dot]) {
                let name = self
                    .consume(
                        TokenType::Identifier("".into()),
                        "Expected property name after '.'",
                    )?
                    .clone();
                expr = Expr::Get(Get::new(expr, name))
            } else {
                break;
            }
        }

        Some(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Option<Expr> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            while {
                if arguments.len() >= 8 {
                    error(self.peek().line, "Cannot have more than 8 arguments");
                }
                arguments.push(self.expression()?);
                self.match_tokens(vec![TokenType::Comma])
            } {}
        }

        let paren = self.consume(
            TokenType::RightParen,
            "Expected ')' after function call arguments",
        )?;
        Some(Expr::Call(Call::new(callee, paren.clone(), arguments)))
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
            TokenType::Identifier(string) => {
                Some(Expr::Variable(Variable::new(self.previous().clone())))
            }
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
