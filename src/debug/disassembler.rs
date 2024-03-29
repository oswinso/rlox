use crate::bytecode::{Chunk, Code, Opcode, Value};
use std::convert::TryInto;
use std::fmt::Write;

use crate::utils::PrettyPrinter;
use crate::vm::Stack;

pub struct Disassembler {
    pretty_printer: PrettyPrinter,
}

impl Disassembler {
    pub fn new() -> Disassembler {
        Disassembler {
            pretty_printer: PrettyPrinter::new(String::new()),
        }
    }

    pub fn result(&self) -> &str {
        self.pretty_printer.result()
    }

    pub fn clear(&mut self) {
        self.pretty_printer.clear();
    }

    pub fn disassemble_chunk(&mut self, chunk: &Chunk, name: &str) {
        self.pretty_printer.begin_chunk(name);

        let mut offset = 0;
        while offset < chunk.code.len() {
            offset = self.disassemble_instruction(&chunk, offset);
            self.pretty_printer.newline();
        }
    }

    pub fn disassemble_instruction(&mut self, chunk: &Chunk, offset: usize) -> usize {
        use Opcode::*;

        self.pretty_printer.chunk_offset(offset);

        if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
            self.pretty_printer.line_number(None);
        } else {
            self.pretty_printer.line_number(Some(chunk.lines[offset]));
        }

        let instruction = chunk.code[offset];
        if let Ok(opcode) = instruction.try_into() {
            match opcode {
                Ret => self.simple(opcode, offset),
                Push => self.offset(opcode, chunk, offset),
                Neg => self.simple(opcode, offset),
                Add | Sub | Mul | Div => self.simple(opcode, offset),
                True | False | Nil | Not => self.simple(opcode, offset),
                Eq | Lt | Gt => self.simple(opcode, offset),
                Print => self.simple(opcode, offset),
                Pop => self.simple(opcode, offset),
                DefineGlobal => self.offset(opcode, chunk, offset),
                GetGlobal | SetGlobal => self.offset(opcode, chunk, offset),
                GetLocal | SetLocal => self.byte(opcode, chunk, offset),
                JZ | JMP => self.jump(opcode, 1, chunk, offset),
                LOOP => self.jump(opcode, -1, chunk, offset)
            }
        } else {
            self.pretty_printer.unknown();
            offset + 1
        }
    }

    fn simple(&mut self, opcode: Opcode, offset: usize) -> usize {
        self.pretty_printer.opcode(opcode);
        offset + 1
    }

    fn offset(&mut self, opcode: Opcode, chunk: &Chunk, offset: usize) -> usize {
        let pointer = chunk.code[offset + 1] as usize;
        let value = &chunk.constants.values[pointer];

        self.pretty_printer.opcode(opcode);
        self.pretty_printer.pointer(pointer);
        self.pretty_printer.value(value);
        offset + 2
    }

    fn jump(&mut self, opcode: Opcode, sign: i32, chunk: &Chunk, offset: usize) -> usize {
        let mut jump: usize = (chunk.code[offset + 1] as usize) << 8;
        jump = jump | chunk.code[offset + 2] as usize;

        self.pretty_printer.opcode(opcode);
        self.pretty_printer.pointer(offset);
        let to_offset = offset as i32 + 3 + sign * (jump as i32);
        self.pretty_printer.chunk_offset(to_offset as usize);
        offset + 3
    }

    fn byte(&mut self, opcode: Opcode, chunk: &Chunk, offset: usize) -> usize {
        let slot = chunk.code[offset + 1];

        self.pretty_printer.opcode(opcode);
        self.pretty_printer.local(slot);
        offset + 2
    }

    pub fn print_stack(&mut self, stack: &Stack) {
        self.pretty_printer.stack(stack);
        self.pretty_printer.newline();
    }

    pub fn print_value(&mut self, value: &Value) {
        self.pretty_printer.value(value);
        self.pretty_printer.newline();
    }
}
