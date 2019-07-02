use std::convert::TryFrom;

use crate::bytecode::Obj;
use std::fmt;

pub(crate) type ConstantPointer = u8;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    Obj(Obj),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(b) => !b,
            Value::Number(n) => *n == 0.0,
            Value::Obj(_) => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(num) => write!(f, "{}", num),
            Value::Obj(obj) => write!(f, "{}", obj),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Nil, Value::Nil) => true,
            (Value::Obj(l), Value::Obj(r)) => l == r,
            _ => false,
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(num) => Ok(num),
            _ => Err(()),
        }
    }
}

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) -> Result<ConstantPointer, ()> {
        match u8::try_from(self.values.len()) {
            Ok(index) => {
                self.values.push(value);
                Ok(index)
            }
            Err(_) => Err(()),
        }
    }
}
