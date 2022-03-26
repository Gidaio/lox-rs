pub const OP_RETURN: u8 = 0;

pub struct Chunk {
    pub code: Vec<u8>,
}

pub fn init_chunk() -> Chunk {
    Chunk { code: vec![] }
}

pub fn free_chunk(_chunk: Chunk) {
    // Just take the chunk and don't give it back.
}

pub fn write_chunk(chunk: &mut Chunk, byte: u8) {
    chunk.code.push(byte);
}
