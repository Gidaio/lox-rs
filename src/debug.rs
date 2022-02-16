use crate::value::print_value;
use crate::{Chunk, OpCode};

impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];

        match instruction.try_into() {
            Ok(OpCode::Constant) => self.constant_instruction("OP_CONSTANT", offset),
            Ok(OpCode::Return) => Self::simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unrecognized instruction {}", instruction);
                offset + 1
            }
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant_index = self.code[offset + 1];
        print!("{:<16} {:4} ", name, constant_index);
        print_value(self.constants[constant_index as usize]);
        println!("");

        offset + 2
    }
}