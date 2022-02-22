mod chunk;
mod debug;
mod opcode;
mod value;
mod vm;

use chunk::Chunk;
use opcode::OpCode;
use vm::VM;

fn main() {
    let mut vm = VM::init();
    let mut chunk = Chunk::init();

    let constant_index = chunk.add_constant(1.2);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write_byte(constant_index as u8, 123);

    let constant_index = chunk.add_constant(3.4);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write_byte(constant_index as u8, 123);

    chunk.write_opcode(OpCode::Add, 123);

    let constant_index = chunk.add_constant(5.6);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write_byte(constant_index as u8, 123);

    chunk.write_opcode(OpCode::Divide, 123);

    chunk.write_opcode(OpCode::Negate, 123);

    chunk.write_opcode(OpCode::Return, 123);

    vm.interpret(chunk);
}
