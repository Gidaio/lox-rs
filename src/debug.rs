use crate::prelude::*;

pub fn disassemble_chunk(chunk: &mut Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &mut Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let instruction = chunk.code[offset];

    match instruction {
        OP_RETURN => simple_instruction("OP_RETURN", offset),
        _ => {
            println!("Unknown opcode {}", instruction);
            offset + 1
        },
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
