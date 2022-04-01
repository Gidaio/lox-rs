mod chunk;
mod common;
mod debug;
mod value;
mod vm;

mod prelude {
    pub use crate::chunk::*;
    pub use crate::common::*;
    pub use crate::debug::*;
    pub use crate::value::*;
    pub use crate::vm::*;
}

use prelude::*;

fn main() {
    let mut vm = init_vm();
    let mut chunk = init_chunk();

    let constant = add_constant(&mut chunk, 1.2);
    write_chunk(&mut chunk, OP_CONSTANT, 123);
    write_chunk(&mut chunk, constant as u8, 123);

    let constant = add_constant(&mut chunk, 3.4);
    write_chunk(&mut chunk, OP_CONSTANT, 123);
    write_chunk(&mut chunk, constant as u8, 123);

    write_chunk(&mut chunk, OP_ADD, 123);

    let constant = add_constant(&mut chunk, 5.6);
    write_chunk(&mut chunk, OP_CONSTANT, 123);
    write_chunk(&mut chunk, constant as u8, 123);

    write_chunk(&mut chunk, OP_DIVIDE, 123);
    write_chunk(&mut chunk, OP_NEGATE, 123);
    write_chunk(&mut chunk, OP_RETURN, 123);

    disassemble_chunk(&mut chunk, "test chunk");
    interpret(&mut vm, &mut chunk);

    free_vm(vm);
    free_chunk(chunk);
}
