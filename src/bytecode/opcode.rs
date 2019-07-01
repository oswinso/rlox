use std::convert::TryFrom;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Ret,
    Push,
    Neg,
    // Binary ops
    Add,
    Sub,
    Mul,
    Div,
    // Literals
    True,
    False,
    Nil,
    // Logical ops
    Not,
    Eq,
    Gt,
    Lt,
    // Print
    Print,
    // Stack
    Pop,
    // Variables
    DefineGlobal,
    GetGlobal,
    SetGlobal,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Opcode::*;
        let string = match self {
            Ret => "RET",
            Push => "PUSH",
            Neg => "NEG",
            Add => "ADD",
            Sub => "SUB",
            Mul => "MUL",
            Div => "DIV",
            True => "TRUE",
            False => "FALSE",
            Nil => "NIL",
            Not => "NOT",
            Eq => "EQ",
            Gt => "GT",
            Lt => "LT",
            Print => "PRINT",
            Pop => "POP",
            DefineGlobal => "DEF_GLOBAL",
            GetGlobal => "GET_GLOBAL",
            SetGlobal => "SET_GLOBAL",
        };
        write!(f, "{}", string)
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        opcode as u8
    }
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        use Opcode::*;

        match byte {
            0 => Ok(Ret),
            1 => Ok(Push),
            2 => Ok(Neg),
            3 => Ok(Add),
            4 => Ok(Sub),
            5 => Ok(Mul),
            6 => Ok(Div),
            7 => Ok(True),
            8 => Ok(False),
            9 => Ok(Nil),
            10 => Ok(Not),
            11 => Ok(Eq),
            12 => Ok(Gt),
            13 => Ok(Lt),
            14 => Ok(Print),
            15 => Ok(Pop),
            16 => Ok(DefineGlobal),
            17 => Ok(GetGlobal),
            18 => Ok(SetGlobal),
            _ => Err(()),
        }
    }
}
