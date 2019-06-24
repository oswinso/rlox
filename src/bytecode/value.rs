pub type Value = f64;

pub(crate) type ConstantPointer = u8;

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) -> ConstantPointer {
        self.values.push(value);
        (self.values.len() - 1) as u8
    }
}
