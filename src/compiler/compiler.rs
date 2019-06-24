use crate::bytecode::{Chunk, Opcode};
use crate::compiler::{CompileError, Parser, Scanner, Source, TokenKind};

pub struct Compiler<'src> {
    source: Source<'src>,
    chunk: Chunk,
    scanner: Scanner<'src>,
    parser: Parser<'src>,
}

pub type CompileResult = Result<Chunk, CompileError>;

impl<'src> Compiler<'src> {
    pub fn new(source: Source<'src>) -> Compiler<'src> {
        Compiler {
            source,
            chunk: Chunk::new(),
            scanner: Scanner::new(source),
            parser: Parser::new(source),
        }
    }

    pub fn compile(&mut self) -> CompileResult {
        self.parser.advance(&mut self.scanner);

        //        self.expression();
        self.parser.consume(
            &mut self.scanner,
            TokenKind::EOF,
            "Expected EOF after expression",
        );
        self.emit_return();

        if self.parser.had_error {
            Err(CompileError {})
        } else {
            Ok(Chunk::new())
        }
    }

    pub fn emit_byte<T>(&mut self, byte: T)
    where
        T: Into<u8>,
    {
        let line = self.parser.previous.as_ref().unwrap().position.line;
        self.chunk.write(byte.into(), line);
    }

    pub fn emit_bytes<T>(&mut self, bytes: &[T])
    where
        T: Into<u8> + Copy,
    {
        let line = self.parser.previous.as_ref().unwrap().position.line;
        for &byte in bytes {
            self.chunk.write(byte.into(), line);
        }
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(Opcode::Ret);
    }
}
