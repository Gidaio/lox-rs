mod chunk;
mod debug;
mod value;

mod prelude {
    pub use crate::chunk::*;
    pub use crate::debug::*;
    pub use crate::value::*;
}

use prelude::*;

fn main() {
    let mut chunk = init_chunk();

    let constant = add_constant(&mut chunk, 1.2);
    write_chunk(&mut chunk, OP_CONSTANT, 123);
    write_chunk(&mut chunk, constant as u8, 123);
    write_chunk(&mut chunk, OP_RETURN, 123);

    disassemble_chunk(&mut chunk, "test chunk");
    free_chunk(chunk);
}
