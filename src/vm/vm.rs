use crate::bytecode::{Chunk, GlobalMap, InternMap, Obj, Opcode, Value};
use crate::vm::errors::*;

use crate::vm::Stack;
use std::convert::TryInto;
use std::iter::Enumerate;
use std::slice::Iter;

#[cfg(feature = "trace_execution")]
use crate::debug::Disassembler;
use crate::utils::PrettyPrinter;
use core::borrow::Borrow;
use std::collections::HashSet;
use std::rc::Rc;

pub type VMResult = Result<(), RuntimeError>;

pub struct VM<'chunk> {
    chunk: &'chunk Chunk,
    ip: Enumerate<Iter<'chunk, u8>>,
    stack: Stack,
    globals: GlobalMap,
    strings: InternMap,

    #[cfg(feature = "trace_execution")]
    disassembler: Disassembler,
    #[cfg(feature = "trace_execution")]
    offset: usize,
}

impl<'chunk> VM<'chunk> {
    pub fn new(chunk: &'chunk Chunk, strings: InternMap) -> VM<'chunk> {
        VM {
            chunk,
            ip: chunk.code.iter().enumerate(),
            stack: Stack::new(),
            globals: GlobalMap::new(),
            strings,
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
            if let Some((line, &instruction)) = self.read_byte() {
                if let Ok(opcode) = instruction.try_into() {
                    match opcode {
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
                                PrettyPrinter::new(String::new()).print_print(&value).print();
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
                                    &format!("Tried to get value of undefined variable '{}'", name.as_ref()),
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
                                    &format!("Tried to assign to undefined variable '{}'", name.as_ref()),
                                ));
                            }
                        }
                    }
                } else {
                    panic!("Couldn't decode opcode {}", instruction);
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
            (Some(Value::Number(left)), Some(Value::Number(right))) => {
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
        let offset = *byte as usize;
        self.chunk.constants.values[offset].clone()
    }

    fn read_byte(&mut self) -> Option<(usize, &u8)> {
        #[cfg(feature = "trace_execution")]
        {
            self.offset += 1;
        }
        self.ip.next()
    }

    fn read_string(&mut self) -> Option<Rc<String>> {
        match self.read_constant() {
            Value::Obj(Obj::String(str)) => Some(str),
            _ => None,
        }
    }
}
