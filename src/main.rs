mod chunk;
mod debug;
mod opcode;
mod value;

use chunk::Chunk;
use opcode::OpCode;

fn main() {
    let mut chunk = Chunk::init();
    let constant = chunk.add_constant(1.2);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write_byte(constant as u8, 123);

    chunk.write_opcode(OpCode::Return, 123);
    chunk.disassemble("test chunk");
}
