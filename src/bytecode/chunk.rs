use crate::bytecode::{ConstantPointer, LineInfoArray, Value, ValueArray};

pub type Code = Vec<u8>;

pub struct Chunk {
    pub code: Code,
    pub constants: ValueArray,
    pub lines: LineInfoArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Code::new(),
            constants: ValueArray::new(),
            lines: LineInfoArray::new(),
        }
    }

    pub fn write<T>(&mut self, byte: T, line: usize)
    where
        T: Into<u8>,
    {
        self.code.push(byte.into());
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> Result<ConstantPointer, ()> {
        self.constants.write(value)
    }
}
