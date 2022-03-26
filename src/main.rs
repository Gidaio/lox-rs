mod chunk;
mod debug;

mod prelude {
    pub use crate::chunk::*;
    pub use crate::debug::*;
}

use prelude::*;

fn main() {
    let mut chunk = init_chunk();
    write_chunk(&mut chunk, OP_RETURN);
    disassemble_chunk(&mut chunk, "test chunk");
    free_chunk(chunk);
}
