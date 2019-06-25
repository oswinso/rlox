use crate::bytecode::{Chunk, Opcode, Value};
use crate::compiler::{CompileError, ParseFn, ParseRule, Parser, Precedence, Scanner, Source, TokenKind, Token};
use std::cell::RefCell;
use crate::utils::PrettyPrinter;

pub struct Compiler<'src> {
    source: Source<'src>,
    chunk: RefCell<Chunk>,
    scanner: RefCell<Scanner<'src>>,
    parser: RefCell<Parser<'src>>,
    pub had_error: bool,
    pub panic_mode: bool,
}

pub type CompileResult = Result<Chunk, CompileError>;

impl<'src> Compiler<'src> {
    pub fn new(source: Source<'src>) -> Compiler<'src> {
        Compiler {
            source,
            chunk: RefCell::new(Chunk::new()),
            scanner: RefCell::new(Scanner::new(source)),
            parser: RefCell::new(Parser::new(source)),
            had_error: false,
            panic_mode: false
        }
    }

    pub fn compile(&mut self) -> CompileResult {
        self.parser.borrow_mut().advance(&mut self.scanner.borrow_mut());

        //        self.expression();
        self.parser.borrow_mut().consume(
            &mut self.scanner.borrow_mut(),
            TokenKind::EOF,
            "Expected EOF after expression",
        );
        self.emit_return();

        if self.had_error {
            Err(CompileError {})
        } else {
            Ok(Chunk::new())
        }
    }

    pub fn number(&mut self) {
        let lexeme = self
            .source
            .get_lexeme(self.parser.borrow().previous.as_ref().unwrap().as_ref());
        let num = match lexeme.parse::<f64>() {
            Ok(num) => num,
            Err(error) => panic!("Tried to parse {} but rip: {}", lexeme, error),
        };
        self.emit_constant(num);
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.parser.borrow_mut().consume(
            &mut self.scanner.borrow_mut(),
            TokenKind::RightParen,
            "Expected '(' after expression.",
        );
    }

    pub fn unary(&mut self) {
        let operator = self.parser.borrow().previous.as_ref().unwrap().ty.clone();

        self.parse_precendence(Precedence::Unary);

        match operator {
            TokenKind::Minus => self.emit_byte(Opcode::Neg),
            _ => unreachable!(),
        }
    }

    pub fn binary(&mut self) {
        let operator = self.parser.borrow().previous.as_ref().unwrap().ty.clone();

        let precedence = self.get_infix_rule(&operator).precedence;
        self.parse_precendence((precedence as u32 + 1).into());

        match operator {
            TokenKind::Plus => self.emit_byte(Opcode::Add),
            TokenKind::Minus => self.emit_byte(Opcode::Add),
            TokenKind::Star => self.emit_byte(Opcode::Mul),
            TokenKind::Slash => self.emit_byte(Opcode::Div),
            _ => unreachable!(),
        }
    }

    pub fn get_grouping(&mut self) -> Option<ParseFn> {
        Some(Box::new(|s: &mut Compiler| s.grouping()))
    }

    pub fn get_unary(&mut self) -> Option<ParseFn> {
        Some(Box::new(|s: &mut Compiler| s.unary()))
    }

    pub fn get_binary(&mut self) -> Option<ParseFn> {
        Some(Box::new(|s: &mut Compiler| s.binary()))
    }

    pub fn get_number(&mut self) -> Option<ParseFn> {
        Some(Box::new(|s: &mut Compiler| s.number()))
    }

    pub fn get_prefix_rule(&mut self, token_kind: &TokenKind) -> ParseRule {
        use super::Keyword::*;
        use TokenKind::*;
        match token_kind {
            LeftParen => ParseRule::new(self.get_grouping(), Precedence::Call),
            RightParen => ParseRule::new(None, Precedence::None),
            LeftBrace => ParseRule::new(None, Precedence::None),
            RightBrace => ParseRule::new(None, Precedence::None),
            Comma => ParseRule::new(None, Precedence::None),
            Dot => ParseRule::new(None, Precedence::Call),
            Minus => ParseRule::new(self.get_unary(), Precedence::Term),
            Plus => ParseRule::new(None, Precedence::Term),
            Semicolon => ParseRule::new(None, Precedence::None),
            Slash => ParseRule::new(None, Precedence::Factor),
            Star => ParseRule::new(None, Precedence::Factor),
            Bang => ParseRule::new(None, Precedence::None),
            BangEqual => ParseRule::new(None, Precedence::Equality),
            Equal => ParseRule::new(None, Precedence::None),
            EqualEqual => ParseRule::new(None, Precedence::Equality),
            Less => ParseRule::new(None, Precedence::Comparison),
            LessEqual => ParseRule::new(None, Precedence::Comparison),
            Greater => ParseRule::new(None, Precedence::Comparison),
            GreaterEqual => ParseRule::new(None, Precedence::Comparison),
            Identifier => ParseRule::new(None, Precedence::None),
            String => ParseRule::new(None, Precedence::None),
            Number => ParseRule::new(self.get_number(), Precedence::None),
            QuestionMark => ParseRule::new(None, Precedence::None),
            Colon => ParseRule::new(None, Precedence::None),
            Keyword(keyword) => match keyword {
                And => ParseRule::new(None, Precedence::And),
                Class => ParseRule::new(None, Precedence::None),
                Else => ParseRule::new(None, Precedence::None),
                False => ParseRule::new(None, Precedence::None),
                Fun => ParseRule::new(None, Precedence::None),
                For => ParseRule::new(None, Precedence::None),
                If => ParseRule::new(None, Precedence::None),
                Nil => ParseRule::new(None, Precedence::None),
                Or => ParseRule::new(None, Precedence::Or),
                Print => ParseRule::new(None, Precedence::None),
                Return => ParseRule::new(None, Precedence::None),
                Super => ParseRule::new(None, Precedence::None),
                This => ParseRule::new(None, Precedence::None),
                True => ParseRule::new(None, Precedence::None),
                Let => ParseRule::new(None, Precedence::None),
                While => ParseRule::new(None, Precedence::None),
            },
            Error(_) => ParseRule::new(None, Precedence::None),
            EOF => ParseRule::new(None, Precedence::None),
        }
    }

    pub fn get_infix_rule<'a>(&self, token_kind: &TokenKind) -> ParseRule {
        ParseRule {
            function: None,
            precedence: Precedence::None
        }
//        use super::Keyword::*;
//        use TokenKind::*;
//        match token_kind {
//            LeftParen => ParseRule::new(Affix::Prefix(self.get_grouping()), Precedence::Call),
//            RightParen => ParseRule::new(None, Precedence::None),
//            LeftBrace => ParseRule::new(None, Precedence::None),
//            RightBrace => ParseRule::new(None, Precedence::None),
//            Comma => ParseRule::new(None, Precedence::None),
//            Dot => ParseRule::new(None, Precedence::Call),
//            Minus => ParseRule::new(Affix::Prefix(self.get_unary()), Precedence::Term),
//            Plus => ParseRule::new(None, self.get_binary(), Precedence::Term),
//            Semicolon => ParseRule::new(None, None, Precedence::None),
//            Slash => ParseRule::new(None, self.get_binary(), Precedence::Factor),
//            Star => ParseRule::new(None, self.get_binary(), Precedence::Factor),
//            Bang => ParseRule::new(None, None, Precedence::None),
//            BangEqual => ParseRule::new(None, None, Precedence::Equality),
//            Equal => ParseRule::new(None, None, Precedence::None),
//            EqualEqual => ParseRule::new(None, None, Precedence::Equality),
//            Less => ParseRule::new(None, None, Precedence::Comparison),
//            LessEqual => ParseRule::new(None, None, Precedence::Comparison),
//            Greater => ParseRule::new(None, None, Precedence::Comparison),
//            GreaterEqual => ParseRule::new(None, None, Precedence::Comparison),
//            Identifier => ParseRule::new(None, None, Precedence::None),
//            String => ParseRule::new(None, None, Precedence::None),
//            Number => ParseRule::new(self.get_number(), None, Precedence::None),
//            QuestionMark => ParseRule::new(None, None, Precedence::None),
//            Colon => ParseRule::new(None, None, Precedence::None),
//            Keyword(keyword) => match keyword {
//                And => ParseRule::new(None, None, Precedence::And),
//                Class => ParseRule::new(None, None, Precedence::None),
//                Else => ParseRule::new(None, None, Precedence::None),
//                False => ParseRule::new(None, None, Precedence::None),
//                Fun => ParseRule::new(None, None, Precedence::None),
//                For => ParseRule::new(None, None, Precedence::None),
//                If => ParseRule::new(None, None, Precedence::None),
//                Nil => ParseRule::new(None, None, Precedence::None),
//                Or => ParseRule::new(None, None, Precedence::Or),
//                Print => ParseRule::new(None, None, Precedence::None),
//                Return => ParseRule::new(None, None, Precedence::None),
//                Super => ParseRule::new(None, None, Precedence::None),
//                This => ParseRule::new(None, None, Precedence::None),
//                True => ParseRule::new(None, None, Precedence::None),
//                Let => ParseRule::new(None, None, Precedence::None),
//                While => ParseRule::new(None, None, Precedence::None),
//            },
//            Error(_) => ParseRule::new(None, None, Precedence::None),
//            EOF => ParseRule::new(None, None, Precedence::None),
//        }
    }

    pub fn expression(&mut self) {
        self.parse_precendence(Precedence::Assignment);
    }

    pub fn parse_precendence(&mut self, precedence: Precedence) {
        self.advance();

        let token_kind = self.parser.borrow().previous.as_ref().unwrap().ty.clone();
        if let Some(prefix_rule) = self.get_prefix_rule(&token_kind).function {
//            prefix_rule();
        } else {
            panic!("Expected expression");
        }
    }

    pub fn emit_byte<T>(&self, byte: T)
    where
        T: Into<u8>,
    {
        let line = self.parser.borrow().previous.as_ref().unwrap().position.line;
        self.chunk.borrow_mut().write(byte.into(), line);
    }

    pub fn emit_bytes<T>(&self, bytes: &[T])
    where
        T: Into<u8> + Copy,
    {
        let line = self.parser.borrow().previous.as_ref().unwrap().position.line;
        for &byte in bytes {
            self.chunk.borrow_mut().write(byte.into(), line);
        }
    }

    pub fn emit_constant(&self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(&[Opcode::Push.into(), constant]);
    }

    pub fn make_constant(&self, value: Value) -> u8 {
        match self.chunk.borrow_mut().add_constant(value) {
            Ok(constant_ptr) => constant_ptr,
            Err(_) => panic!("TODO: lmao go fix this error. Too many constants here"),
        }
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(Opcode::Ret);
    }

    fn advance(&mut self) {
         if self.parser.borrow_mut().advance(&mut self.scanner.borrow_mut()).is_err() {
            self.error_at_current()
        }
    }

    fn error_at_current(&mut self) {
        let token = self.parser.borrow().current.as_ref().unwrap().clone();
        let error_message = token.ty.try_into_error().unwrap();
        self.parser_error(error_message);
    }

    fn parser_error(&mut self, error_message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        let token = self.parser.borrow().current.as_ref().unwrap().clone();
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
}
