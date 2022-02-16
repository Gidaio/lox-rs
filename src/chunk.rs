use crate::value::Value;
use crate::OpCode;

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn init() -> Self {
        Self {
            code: vec![],
            lines: vec![],
            constants: vec![],
        }
    }

    pub fn write_opcode(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte as u8);
        self.lines.push(line);
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
