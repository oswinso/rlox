use crate::compiler::{Scanner, Source, Token, TokenKind};
use crate::utils::PrettyPrinter;
use core::borrow::Borrow;
use std::rc::Rc;

pub struct Parser<'a> {
    source: Source<'a>,
    pub current: Option<Rc<Token>>,
    pub previous: Option<Rc<Token>>,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl<'a> Parser<'a> {
    pub fn new(source: Source<'a>) -> Parser<'a> {
        Parser {
            source,
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self, scanner: &mut Scanner<'a>) -> Result<(), Token> {
        self.previous = self.current.clone();

        loop {
            self.current = Some(Rc::new(scanner.scan_token()));
            if self.current.as_ref().unwrap().ty == TokenKind::error_type() {
                self.error_at_current();
            } else {
                break;
            }
        }
        Ok(())
    }

    fn error_at_current(&mut self) {
        let token = self.current.as_ref().unwrap().clone();
        let error_message = token.ty.try_into_error().unwrap();
        self.parser_error(error_message);
    }

    fn parser_error(&mut self, error_message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        let token = self.current.as_ref().unwrap().clone();
        self.error(&token, error_message)
    }

    fn error(&mut self, token: &Token, message: &str) {
        let lexeme = self.source.get_lexeme(&token);
        PrettyPrinter::new(String::new())
            .parse_error(token, lexeme, message)
            .newline()
            .print();
        self.had_error = true;
    }

    pub fn consume(&mut self, scanner: &mut Scanner<'a>, token_kind: TokenKind, message: &str) {
        if self.current.as_ref().unwrap().ty == token_kind {
            self.advance(scanner);
        } else {
            self.parser_error(message);
        }
    }
}
