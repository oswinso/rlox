use crate::bytecode::{Opcode, Value};
use crate::vm::Stack;
use ansi_term::Color;
use std::fmt::Write;

pub struct PrettyPrinter {
    string: String,
    label: Color,
    error: Color,
    chunk_offset: Color,
    line_number: Color,
    opcode: Color,
    offset: Color,
    value: Color,
}

impl PrettyPrinter {
    pub fn new(string: String) -> PrettyPrinter {
        PrettyPrinter {
            string,
            label: Color::RGB(203, 75, 22),
            error: Color::RGB(220, 50, 47),
            chunk_offset: Color::RGB(101, 123, 131),
            line_number: Color::RGB(101, 123, 131),
            opcode: Color::RGB(211, 54, 130),
            offset: Color::RGB(131, 148, 150),
            value: Color::RGB(133, 153, 0),
        }
    }

    pub fn begin_chunk(&mut self, chunk_name: &str) {
        let format = format!("===== {:^12} =====", chunk_name);
        writeln!(self.string, "{}", self.label.paint(format)).unwrap();
    }

    pub fn unknown(&mut self) {
        write!(self.string, "{}", self.error.paint("Bad opcode")).unwrap();
    }

    pub fn chunk_offset(&mut self, offset: usize) {
        let format = format!("{:04X} ", offset);
        write!(self.string, "{}", self.chunk_offset.paint(format)).unwrap();
    }

    pub fn line_number(&mut self, line_number: Option<usize>) {
        let format = if let Some(line_number) = line_number {
            format!("{:4}{:4} ", line_number, "")
        } else {
            "   |     ".into()
        };
        write!(self.string, "{}", self.line_number.paint(format)).unwrap();
    }

    pub fn opcode(&mut self, opcode: Opcode) {
        let format = format!("{:4}{:4}", opcode, "");
        write!(self.string, "{}", self.opcode.paint(format)).unwrap();
    }

    pub fn newline(&mut self) {
        write!(self.string, "\n").unwrap();
    }

    pub fn pointer(&mut self, pointer: usize) {
        let format = format!("{:04X} -> ", pointer);
        write!(self.string, "{}", self.offset.paint(format)).unwrap();
    }

    pub fn value(&mut self, value: &Value) {
        let format = format!("{:4.2} ", value);
        write!(self.string, "{}", self.value.paint(format)).unwrap();
    }

    pub fn result(&self) -> &str {
        &self.string
    }

    pub fn clear(&mut self) {
        self.string.clear();
    }

    pub fn stack(&mut self, stack: &Stack) {
        write!(self.string, "{:4}{{ ", "");
        for value in stack {
            write!(self.string, "[ ");
            self.value(value);
            write!(self.string, " ]");
        }
        write!(self.string, " }}");
    }
}
