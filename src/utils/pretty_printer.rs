use crate::bytecode::{Opcode, Value};
use crate::vm::Stack;
use ansi_term::{Color, Style};
use std::fmt::Write;
use std::io;
use std::io::Write as IOWrite;
use crate::compiler::Token;

pub struct PrettyPrinter {
    string: String,
    label: Style,
    error: Style,
    chunk_offset: Style,
    line_number: Style,
    opcode: Style,
    offset: Style,
    value: Style,
    prompt: Style,
}

impl PrettyPrinter {
    pub fn new(string: String) -> PrettyPrinter {
        PrettyPrinter {
            string,
            label: Color::RGB(203, 75, 22).into(), // orange
            error: Color::RGB(220, 50, 47).bold(), // red
            chunk_offset: Color::RGB(101, 123, 131).into(), // base00
            line_number: Color::RGB(101, 123, 131).into(), // base00
            opcode: Color::RGB(211, 54, 130).bold(), // magenta
            offset: Color::RGB(131, 148, 150).into(), // base0
            value: Color::RGB(133, 153, 0).into(), // green
            prompt: Color::RGB(38, 139, 210).bold(), // blue
        }
    }

    pub fn begin_chunk(&mut self, chunk_name: &str) -> &mut Self {
        let format = format!("===== {:^12} =====", chunk_name);
        writeln!(self.string, "{}", self.label.paint(format)).unwrap();
        self
    }

    pub fn unknown(&mut self) -> &mut Self {
        write!(self.string, "{}", self.error.paint("Bad opcode")).unwrap();
        self
    }

    pub fn chunk_offset(&mut self, offset: usize) -> &mut Self {
        let format = format!("{:04X} ", offset);
        write!(self.string, "{}", self.chunk_offset.paint(format)).unwrap();
        self
    }

    pub fn line_number(&mut self, line_number: Option<usize>) -> &mut Self {
        let format = if let Some(line_number) = line_number {
            format!("{:4}{:4} ", line_number, "")
        } else {
            "   |     ".into()
        };
        write!(self.string, "{}", self.line_number.paint(format)).unwrap();
        self
    }

    pub fn opcode(&mut self, opcode: Opcode) -> &mut Self {
        let format = format!("{:4}{:4}", opcode, "");
        write!(self.string, "{}", self.opcode.paint(format)).unwrap();
        self
    }

    pub fn newline(&mut self) -> &mut Self {
        write!(self.string, "\n").unwrap();
        self
    }

    pub fn pointer(&mut self, pointer: usize) -> &mut Self {
        let format = format!("{:04X} -> ", pointer);
        write!(self.string, "{}", self.offset.paint(format)).unwrap();
        self
    }

    pub fn value(&mut self, value: &Value) -> &mut Self {
        let format = format!("{:4.2} ", value);
        write!(self.string, "{}", self.value.paint(format)).unwrap();
        self
    }

    pub fn result(&self) -> &str {
        &self.string
    }

    pub fn clear(&mut self) -> &mut Self {
        self.string.clear();
        self
    }

    pub fn stack(&mut self, stack: &Stack) -> &mut Self {
        write!(self.string, "{:4}{{ ", "").unwrap();
        for value in stack {
            write!(self.string, "[ ").unwrap();
            self.value(value);
            write!(self.string, " ]").unwrap();
        }
        write!(self.string, " }}").unwrap();
        self
    }

    pub fn prompt(&mut self) -> &mut Self {
        write!(self.string, "{}", self.prompt.paint("> ")).unwrap();
        self
    }

    pub fn print(&mut self) -> &mut Self {
        print!("{}", self.string);
        io::stdout().flush().unwrap();
        self.string.clear();
        self
    }

    pub fn token(&mut self, token: &Token) -> &mut Self {
        let format = format!("{:?}", token.ty);
        write!(self.string, "{}", self.opcode.paint(format)).unwrap();
        self
    }
}
