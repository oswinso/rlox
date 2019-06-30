use crate::bytecode::{Chunk, Opcode, Value, Obj};
use crate::compiler::{
    CompileError, Keyword, ParseFn, ParseRule, Parser, Precedence, Scanner, Source, Token,
    TokenKind,
};
use crate::utils::PrettyPrinter;
use std::fmt::Debug;

#[cfg(feature = "print_code")]
use crate::debug::Disassembler;
use std::rc::Rc;

pub struct Compiler<'src> {
    source: Source<'src>,
    chunk: Chunk,
    scanner: Scanner<'src>,
    parser: Parser<'src>,
    pub had_error: bool,
    pub panic_mode: bool,
}

pub type CompileResult = Result<Chunk, CompileError>;

impl<'src> Compiler<'src> {
    pub fn new(source: Source<'src>) -> Compiler<'src> {
        Compiler {
            source,
            chunk: Chunk::new(),
            scanner: Scanner::new(source),
            parser: Parser::new(source),
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn compile(mut self) -> CompileResult {
        self.parser.advance(&mut self.scanner).unwrap();

        self.expression();
        self.parser.consume(
            &mut self.scanner,
            TokenKind::EOF,
            "Expected EOF after expression",
        );
        self.emit_return();

        #[cfg(feature = "print_code")]
        {
            let mut d = Disassembler::new();
            d.disassemble_chunk(&self.chunk, "lol");
            println!("{}", d.result());
            d.clear();
        }

        if self.had_error {
            Err(CompileError {})
        } else {
            Ok(self.chunk)
        }
    }

    pub fn number(&mut self) {
        let lexeme = self
            .source
            .get_lexeme(self.parser.previous.as_ref().unwrap().as_ref());
        let num = match lexeme.parse::<f64>() {
            Ok(num) => num,
            Err(error) => panic!("Tried to parse {} but rip: {}", lexeme, error),
        };
        self.emit_constant(Value::Number(num));
    }

    pub fn literal(&mut self) {
        match self.get_previous().ty {
            TokenKind::Keyword(Keyword::True) => self.emit_byte(Opcode::True),
            TokenKind::Keyword(Keyword::False) => self.emit_byte(Opcode::False),
            TokenKind::Keyword(Keyword::Nil) => self.emit_byte(Opcode::Nil),
            _ => unreachable!(),
        }
    }

    pub fn string(&mut self) {
        let copied_string = self.source.get_string(self.parser.previous.as_ref().unwrap()).to_owned();
        self.emit_constant(Value::Obj(Obj::String(Rc::new(copied_string))))
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.parser.consume(
            &mut self.scanner,
            TokenKind::RightParen,
            "Expected '(' after expression.",
        );
    }

    pub fn unary(&mut self) {
        let operator = self.parser.previous.as_ref().unwrap().ty.clone();

        self.parse_precendence(Precedence::Unary);

        match operator {
            TokenKind::Minus => self.emit_byte(Opcode::Neg),
            TokenKind::Bang => self.emit_byte(Opcode::Not),
            _ => unreachable!(),
        }
    }

    pub fn binary(&mut self) {
        let operator = self.parser.previous.as_ref().unwrap().ty.clone();

        let precedence = self.get_infix_rule(&operator).precedence;
        self.parse_precendence((precedence as u32 + 1).into());

        match operator {
            TokenKind::Plus => self.emit_byte(Opcode::Add),
            TokenKind::Minus => self.emit_byte(Opcode::Sub),
            TokenKind::Star => self.emit_byte(Opcode::Mul),
            TokenKind::Slash => self.emit_byte(Opcode::Div),
            TokenKind::BangEqual => self.emit_bytes(&vec![Opcode::Eq, Opcode::Not]),
            TokenKind::EqualEqual => self.emit_byte(Opcode::Eq),
            TokenKind::GreaterEqual => self.emit_bytes(&vec![Opcode::Lt, Opcode::Not]),
            TokenKind::Greater => self.emit_byte(Opcode::Gt),
            TokenKind::LessEqual => self.emit_bytes(&vec![Opcode::Gt, Opcode::Not]),
            TokenKind::Less => self.emit_byte(Opcode::Lt),
            _ => unreachable!(),
        };
    }

    pub fn get_grouping<'a>() -> Option<ParseFn<'a>> {
        Some(Box::new(|s: &mut Compiler| s.grouping()))
    }

    pub fn get_unary<'a>() -> Option<ParseFn<'a>> {
        Some(Box::new(|s: &mut Compiler| s.unary()))
    }

    pub fn get_binary<'a>() -> Option<ParseFn<'a>> {
        Some(Box::new(|s: &mut Compiler| s.binary()))
    }

    pub fn get_number<'a>() -> Option<ParseFn<'a>> {
        Some(Box::new(|s: &mut Compiler| s.number()))
    }

    pub fn get_literal<'a>() -> Option<ParseFn<'a>> {
        Some(Box::new(|s: &mut Compiler| s.literal()))
    }

    pub fn get_string<'a>() -> Option<ParseFn<'a>> {
        Some(Box::new(|s: &mut Compiler| s.string()))
    }

    pub fn get_prefix_rule<'a>(&self, token_kind: &TokenKind) -> ParseRule<'a> {
        use super::Keyword::*;
        use TokenKind::*;
        match token_kind {
            LeftParen => ParseRule::new(Compiler::get_grouping(), Precedence::Call),
            RightParen => ParseRule::new(None, Precedence::None),
            LeftBrace => ParseRule::new(None, Precedence::None),
            RightBrace => ParseRule::new(None, Precedence::None),
            Comma => ParseRule::new(None, Precedence::None),
            Dot => ParseRule::new(None, Precedence::Call),
            Minus => ParseRule::new(Compiler::get_unary(), Precedence::Term),
            Plus => ParseRule::new(None, Precedence::Term),
            Semicolon => ParseRule::new(None, Precedence::None),
            Slash => ParseRule::new(None, Precedence::Factor),
            Star => ParseRule::new(None, Precedence::Factor),
            Bang => ParseRule::new(Compiler::get_unary(), Precedence::None),
            BangEqual => ParseRule::new(None, Precedence::Equality),
            Equal => ParseRule::new(None, Precedence::None),
            EqualEqual => ParseRule::new(None, Precedence::Equality),
            Less => ParseRule::new(None, Precedence::Comparison),
            LessEqual => ParseRule::new(None, Precedence::Comparison),
            Greater => ParseRule::new(None, Precedence::Comparison),
            GreaterEqual => ParseRule::new(None, Precedence::Comparison),
            Identifier => ParseRule::new(None, Precedence::None),
            String => ParseRule::new(Compiler::get_string(), Precedence::None),
            Number => ParseRule::new(Compiler::get_number(), Precedence::None),
            QuestionMark => ParseRule::new(None, Precedence::None),
            Colon => ParseRule::new(None, Precedence::None),
            Keyword(keyword) => match keyword {
                And => ParseRule::new(None, Precedence::And),
                Class => ParseRule::new(None, Precedence::None),
                Else => ParseRule::new(None, Precedence::None),
                False => ParseRule::new(Compiler::get_literal(), Precedence::None),
                Fun => ParseRule::new(None, Precedence::None),
                For => ParseRule::new(None, Precedence::None),
                If => ParseRule::new(None, Precedence::None),
                Nil => ParseRule::new(Compiler::get_literal(), Precedence::None),
                Or => ParseRule::new(None, Precedence::Or),
                Print => ParseRule::new(None, Precedence::None),
                Return => ParseRule::new(None, Precedence::None),
                Super => ParseRule::new(None, Precedence::None),
                This => ParseRule::new(None, Precedence::None),
                True => ParseRule::new(Compiler::get_literal(), Precedence::None),
                Let => ParseRule::new(None, Precedence::None),
                While => ParseRule::new(None, Precedence::None),
            },
            Error(_) => ParseRule::new(None, Precedence::None),
            EOF => ParseRule::new(None, Precedence::None),
        }
    }

    pub fn get_infix_rule<'a>(&self, token_kind: &TokenKind) -> ParseRule<'a> {
        use super::Keyword::*;
        use TokenKind::*;
        match token_kind {
            LeftParen => ParseRule::new(None, Precedence::Call),
            RightParen => ParseRule::new(None, Precedence::None),
            LeftBrace => ParseRule::new(None, Precedence::None),
            RightBrace => ParseRule::new(None, Precedence::None),
            Comma => ParseRule::new(None, Precedence::None),
            Dot => ParseRule::new(None, Precedence::Call),
            Minus => ParseRule::new(Compiler::get_binary(), Precedence::Term),
            Plus => ParseRule::new(Compiler::get_binary(), Precedence::Term),
            Semicolon => ParseRule::new(None, Precedence::None),
            Slash => ParseRule::new(Compiler::get_binary(), Precedence::Factor),
            Star => ParseRule::new(Compiler::get_binary(), Precedence::Factor),
            Bang => ParseRule::new(None, Precedence::None),
            BangEqual => ParseRule::new(Compiler::get_binary(), Precedence::Equality),
            Equal => ParseRule::new(None, Precedence::None),
            EqualEqual => ParseRule::new(Compiler::get_binary(), Precedence::Equality),
            Less => ParseRule::new(Compiler::get_binary(), Precedence::Comparison),
            LessEqual => ParseRule::new(Compiler::get_binary(), Precedence::Comparison),
            Greater => ParseRule::new(Compiler::get_binary(), Precedence::Comparison),
            GreaterEqual => ParseRule::new(Compiler::get_binary(), Precedence::Comparison),
            Identifier => ParseRule::new(None, Precedence::None),
            String => ParseRule::new(None, Precedence::None),
            Number => ParseRule::new(Compiler::get_number(), Precedence::None),
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

    pub fn expression(&mut self) {
        self.parse_precendence(Precedence::Assignment);
    }

    pub fn parse_precendence(&mut self, precedence: Precedence) {
        self.advance();

        if let Some(prefix_rule) = self.get_prefix_rule(&self.get_previous().ty).function {
            prefix_rule(self);
        } else {
            return;
        };

        let mut other_precedence = self.get_infix_rule(&self.get_current().ty).precedence;
        while precedence <= other_precedence {
            self.advance();
            if let Some(infix_rule) = self.get_infix_rule(&self.get_previous().ty).function {
                infix_rule(self);
            }

            other_precedence = self.get_infix_rule(&self.get_current().ty).precedence;
        }
    }

    fn get_previous(&self) -> &Token {
        &self.parser.previous.as_ref().unwrap()
    }

    fn get_current(&self) -> &Token {
        &self.parser.current.as_ref().unwrap()
    }

    pub fn emit_byte<T>(&mut self, byte: T)
    where
        T: Into<u8> + Debug,
    {
        let line = self.get_previous().position.line;
        self.chunk.write(byte.into(), line);
    }

    pub fn emit_bytes<T>(&mut self, bytes: &[T])
    where
        T: Into<u8> + Copy,
    {
        let line = self.get_previous().position.line;
        for &byte in bytes {
            self.chunk.write(byte.into(), line);
        }
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(&[Opcode::Push.into(), constant]);
    }

    pub fn make_constant(&mut self, value: Value) -> u8 {
        match self.chunk.add_constant(value) {
            Ok(constant_ptr) => constant_ptr,
            Err(_) => panic!("TODO: lmao go fix this error. Too many constants here"),
        }
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(Opcode::Ret);
    }

    fn advance(&mut self) {
        if self.parser.advance(&mut self.scanner).is_err() {
            self.parser_error()
        }
    }

    fn parser_error(&mut self) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        let error_message = self
            .parser
            .current
            .as_ref()
            .unwrap()
            .ty
            .try_into_error()
            .unwrap();
        let token = self.parser.current.as_ref().unwrap().clone();

        let lexeme = self.source.get_lexeme(&token);
        Compiler::error(lexeme, &mut self.had_error, &token, error_message)
    }

    fn error(lexeme: &str, error: &mut bool, token: &Token, message: &str) {
        PrettyPrinter::new(String::new())
            .parse_error(token, lexeme, message)
            .newline()
            .print();
        *error = true;
    }
}
