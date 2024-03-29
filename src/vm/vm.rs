use crate::bytecode::{Chunk, GlobalMap, InternMap, Obj, Opcode, Value, LocalMap};
use crate::vm::errors::*;

use crate::vm::Stack;
use std::convert::TryInto;
use std::iter::Enumerate;
use std::slice::Iter;

#[cfg(feature = "trace_execution")]
use crate::debug::Disassembler;
#[cfg(feature = "trace_execution")]
use crate::utils::PrettyPrinter;
use std::rc::Rc;

pub type VMResult = Result<(), RuntimeError>;

pub struct VM<'chunk> {
    chunk: &'chunk Chunk,
    ip: usize,
    stack: Stack,
    globals: GlobalMap,
    strings: InternMap,
    locals: LocalMap,

    #[cfg(feature = "trace_execution")]
    disassembler: Disassembler,
    #[cfg(feature = "trace_execution")]
    offset: usize,
}

impl<'chunk> VM<'chunk> {
    pub fn new(chunk: &'chunk Chunk, strings: InternMap, locals: LocalMap) -> VM<'chunk> {
        VM {
            chunk,
            ip: 0,
            stack: Stack::new(),
            globals: GlobalMap::new(),
            strings,
            locals,
            #[cfg(feature = "trace_execution")]
            disassembler: Disassembler::new(),
            #[cfg(feature = "trace_execution")]
            offset: 0,
        }
    }

    pub fn interpret(&mut self) -> VMResult {
        use Opcode::*;

        loop {
            #[cfg(feature = "trace_execution")]
            {
                self.disassembler.print_stack(&self.stack);
                self.disassembler
                    .disassemble_instruction(self.chunk, self.offset);
                println!("{}", self.disassembler.result());
                self.disassembler.clear();
            }
            if let Some((line, instruction)) = self.read_byte() {
                match instruction.try_into() {
                    Ok(opcode) => match opcode {
                        Ret => {
                            #[cfg(feature = "trace_execution")]
                            {
                                let ret = self.stack.pop().unwrap_or(Value::Nil);
                                self.disassembler.print_value(&ret);
                                println!("{}", self.disassembler.result());
                                self.disassembler.clear();
                            }
                            return Ok(());
                        }
                        Push => {
                            let constant = self.read_constant();
                            self.stack.push(constant);
                        }
                        Neg => {
                            let val: f64 = {
                                match self.stack.last().unwrap() {
                                    Value::Number(_) => {
                                        self.stack.pop().unwrap().try_into().unwrap()
                                    }
                                    _ => {
                                        return Err(RuntimeError::new(
                                            line,
                                            "Operand must be a number",
                                        ));
                                    }
                                }
                            };
                            self.stack.push(Value::Number(val));
                        }
                        Add => self.add()?,
                        Sub => self.binary_op(|left, right| Value::Number(left - right))?,
                        Mul => self.binary_op(|left, right| Value::Number(left * right))?,
                        Div => self.binary_op(|left, right| Value::Number(left / right))?,
                        True => self.stack.push(Value::Bool(true)),
                        False => self.stack.push(Value::Bool(false)),
                        Nil => self.stack.push(Value::Nil),
                        Not => {
                            let is_falsey = self.stack.pop().unwrap().is_falsey();
                            self.stack.push(Value::Bool(is_falsey))
                        }
                        Eq => {
                            let a = self.stack.pop().unwrap();
                            let b = self.stack.pop().unwrap();
                            self.stack.push(Value::Bool(a == b))
                        }
                        Gt => self.binary_op(|left, right| Value::Bool(left > right))?,
                        Lt => self.binary_op(|left, right| Value::Bool(left < right))?,
                        Print => {
                            let value = self.stack.pop().unwrap();
                            let mut debug = false;
                            #[cfg(feature = "trace_execution")]
                            {
                                debug = true;
                                PrettyPrinter::new(String::new())
                                .print_print(&value)
                                .print();
                            }
                            if !debug {
                                println!("{}", &value);
                            }
                        }
                        Pop => {
                            self.stack.pop().unwrap();
                        }
                        DefineGlobal => {
                            let name = self.read_string().unwrap().as_ref().to_owned();
                            self.globals.insert(name, self.stack.pop().unwrap());
                        }
                        GetGlobal => {
                            let name = self.read_string().unwrap();
                            if let Some(value) = self.globals.get(name.as_ref()) {
                                self.stack.push(value.clone());
                            } else {
                                return Err(RuntimeError::new(
                                    line,
                                    &format!(
                                        "Tried to get value of undefined variable '{}'",
                                        name.as_ref()
                                    ),
                                ));
                            }
                        }
                        SetGlobal => {
                            let name = self.read_string().unwrap();
                            if let Some(entry) = self.globals.get_mut(name.as_ref()) {
                                *entry = self.stack.last().unwrap().clone();
                            } else {
                                return Err(RuntimeError::new(
                                    line,
                                    &format!(
                                        "Tried to assign to undefined variable '{}'",
                                        name.as_ref()
                                    ),
                                ));
                            }
                        }
                        GetLocal => {
                            if let Some((_line, offset)) = self.read_byte() {
                                self.stack.push(self.stack[offset as usize].clone())
                            }
                        }
                        SetLocal => {
                            if let Some((_line, offset)) = self.read_byte() {
                                self.stack[offset as usize] = self.stack.last().unwrap().clone();
                            }
                        }
                        JZ => {
                            if let Some((_line, offset)) = self.read_short() {
                                if self.stack.last().as_ref().unwrap().is_falsey() {
                                    self.move_ip(offset as i32);
                                }
                            }
                        }
                        JMP => {
                            if let Some((_line, offset)) = self.read_short() {
                                self.move_ip(offset as i32);
                            }
                        }
                        LOOP => {
                            if let Some((_line, offset)) = self.read_short() {
                                self.move_ip(-(offset as i32));
                            }
                        }
                    },
                    Err(..) => {
                        panic!("Couldn't decode opcode {}", instruction);
                    }
                }
            } else {
                return Ok(());
            }
        }
    }

    fn binary_op<F>(&mut self, f: F) -> VMResult
    where
        F: FnOnce(f64, f64) -> Value,
    {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Value::Number(right)), Some(Value::Number(left))) => {
                self.stack.push(f(left, right));
                Ok(())
            }
            (Some(_), Some(_)) => {
                return Err(RuntimeError::new(0, "Expected two numbers on the stack"));
            }
            (None, _) | (_, None) => {
                return Err(RuntimeError::new(
                    0,
                    "Expected at least two items on the stack",
                ));
            }
        }
    }

    fn add(&mut self) -> VMResult {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Value::Number(left)), Some(Value::Number(right))) => {
                self.stack.push(Value::Number(left + right));
                Ok(())
            }
            (Some(Value::Obj(Obj::String(second))), Some(Value::Obj(Obj::String(first)))) => {
                self.concatenate_strings(first, second)
            }
            (Some(_), Some(_)) => {
                return Err(RuntimeError::new(
                    0,
                    "Expected two numbers or two strings on the stack",
                ));
            }
            (None, _) | (_, None) => {
                return Err(RuntimeError::new(
                    0,
                    "Expected at least two items on the stack",
                ));
            }
        }
    }

    fn concatenate_strings(&mut self, first: Rc<String>, second: Rc<String>) -> VMResult {
        let concat = format!("{}{}", &first, &second);
        let rc_concat = if let Some(string) = self.strings.get(&concat) {
            string.clone()
        } else {
            let rc = Rc::new(concat.clone());
            self.strings.insert(concat, rc.clone());
            rc
        };
        self.stack.push(Value::Obj(Obj::String(rc_concat)));
        Ok(())
    }

    fn read_constant(&mut self) -> Value {
        let (_line, byte) = self.read_byte().unwrap();
        let offset = byte as usize;
        self.chunk.constants.values[offset].clone()
    }

    fn read_byte(&mut self) -> Option<(usize, u8)> {
        #[cfg(feature = "trace_execution")]
        {
            self.offset += 1;
        }
        let ret = Some((self.ip, self.chunk.code[self.ip]));
        self.ip += 1;
        ret
    }

    fn read_short(&mut self) -> Option<(usize, u16)> {
        let (line, top) = self.read_byte()?;
        let (_, bottom) = self.read_byte()?;
        let concat = ((top as u16) << 8) + bottom as u16;
        Some((line, concat))
    }

    fn read_string(&mut self) -> Option<Rc<String>> {
        match self.read_constant() {
            Value::Obj(Obj::String(str)) => Some(str),
            _ => None,
        }
    }

    fn move_ip(&mut self, offset: i32) {
        #[cfg(feature = "trace_execution")]
        {
            self.offset = ((self.offset as i32) + offset) as usize;
        }
        self.ip = (self.ip as i32 + offset) as usize;
    }
}
