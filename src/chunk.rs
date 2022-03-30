use crate::prelude::*;

pub const OP_CONSTANT: u8 = 0;
pub const OP_RETURN: u8 = 1;

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

pub fn init_chunk() -> Chunk {
    Chunk {
        code: vec![],
        lines: vec![],
        constants: vec![],
    }
}

pub fn free_chunk(_chunk: Chunk) {
    // Just take the chunk and don't give it back.
}

pub fn write_chunk(chunk: &mut Chunk, byte: u8, line: usize) {
    chunk.code.push(byte);
    chunk.lines.push(line);
}

pub fn add_constant(chunk: &mut Chunk, value: Value) -> usize {
    chunk.constants.push(value);
    chunk.constants.len() - 1
}
