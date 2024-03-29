use crate::compiler::{Compiler, Precedence, Scanner, Source, Token, TokenKind};
use std::rc::Rc;

pub struct Parser<'a> {
    source: Source<'a>,
    pub current: Option<Rc<Token>>,
    pub previous: Option<Rc<Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: Source<'a>) -> Parser<'a> {
        Parser {
            source,
            current: None,
            previous: None,
        }
    }

    pub fn advance(&mut self, scanner: &mut Scanner<'a>) -> Result<(), ()> {
        self.previous = self.current.clone();

        loop {
            self.current = Some(Rc::new(scanner.scan_token()));
            if self.current.as_ref().unwrap().ty == TokenKind::error_type() {
                return Err(());
            } else {
                break;
            }
        }
        Ok(())
    }

    pub fn consume(&mut self, scanner: &mut Scanner<'a>, token_kind: &TokenKind, message: &str) {
        if self.check(&token_kind) {
            self.advance(scanner).unwrap();
        } else {
            eprintln!("Error: {}", message);
            //            self.parser_error(message);
        }
    }

    pub fn try_consume(&mut self, scanner: &mut Scanner<'a>, token_kind: &TokenKind) -> bool {
        if self.check(&token_kind) {
            self.consume(scanner, &token_kind, "");
            true
        } else {
            false
        }
    }

    pub fn check(&self, token_kind: &TokenKind) -> bool {
        &self.current.as_ref().unwrap().ty == token_kind
    }
}

pub type ParseFn<'a> = Box<dyn FnOnce(&mut Compiler, bool) -> () + 'a>;

pub struct ParseRule<'a> {
    pub function: Option<ParseFn<'a>>,
    pub precedence: Precedence,
}

impl<'a> ParseRule<'a> {
    pub fn new(function: Option<ParseFn<'a>>, precedence: Precedence) -> ParseRule<'a> {
        ParseRule {
            function,
            precedence,
        }
    }
}
