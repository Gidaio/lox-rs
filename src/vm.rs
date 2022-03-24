use crate::chunk::Chunk;
use crate::compiler::Parser;
use crate::opcode::OpCode;
use crate::value::print_value;
use crate::value::Value;

const DEBUG_TRACE_EXECUTION: bool = true;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn init() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: vec![],
        }
    }

    pub fn interpret(source: String) -> InterpretResult {
        let mut parser = Parser::init(source);
        parser.compile();
        InterpretResult::Ok
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG_TRACE_EXECUTION {
                println!("{:?}", self.stack);
                self.disassemble_instruction();
            }
            let instruction = self.read_byte();
            match instruction.try_into() {
                Ok(OpCode::Constant) => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                Ok(OpCode::Add) => {
                    if let Some(b) = self.stack.pop() {
                        if let Some(a) = self.stack.pop() {
                            self.stack.push(a + b);
                        } else {
                            panic!("Got an add opcode, but there was only one thing on the stack.");
                        }
                    } else {
                        panic!("Got an add opcode, but there was nothing on the stack.");
                    }
                }
                Ok(OpCode::Subtract) => {
                    if let Some(b) = self.stack.pop() {
                        if let Some(a) = self.stack.pop() {
                            self.stack.push(a - b);
                        } else {
                            panic!(
                                "Got a subtract opcode, but there was only one thing on the stack."
                            );
                        }
                    } else {
                        panic!("Got a subtract opcode, but there was nothing on the stack.");
                    }
                }
                Ok(OpCode::Multiply) => {
                    if let Some(b) = self.stack.pop() {
                        if let Some(a) = self.stack.pop() {
                            self.stack.push(a * b);
                        } else {
                            panic!(
                                "Got a multiply opcode, but there was only one thing on the stack."
                            );
                        }
                    } else {
                        panic!("Got a multiply opcode, but there was nothing on the stack.");
                    }
                }
                Ok(OpCode::Divide) => {
                    if let Some(b) = self.stack.pop() {
                        if let Some(a) = self.stack.pop() {
                            self.stack.push(a / b);
                        } else {
                            panic!(
                                "Got a divide opcode, but there was only one thing on the stack."
                            );
                        }
                    } else {
                        panic!("Got a divide opcode, but there was nothing on the stack.");
                    }
                }
                Ok(OpCode::Negate) => {
                    if let Some(value) = self.stack.pop() {
                        self.stack.push(-value);
                    } else {
                        panic!("Got a negate opcode, but there was nothing on the stack.");
                    }
                }
                Ok(OpCode::Return) => {
                    if let Some(value) = self.stack.pop() {
                        print_value(value);
                        println!("");
                    } else {
                        panic!("Stack is empty!");
                    }
                    return InterpretResult::Ok;
                }
                _ => panic!("Oh noes!"),
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        if let Some(chunk) = &self.chunk {
            let byte = chunk.code[self.ip];
            self.ip += 1;
            byte
        } else {
            panic!("Aah! No chunk!");
        }
    }

    fn read_constant(&mut self) -> f64 {
        let constant_index = self.read_byte() as usize;
        if let Some(chunk) = &self.chunk {
            chunk.constants[constant_index]
        } else {
            panic!("No chunk!");
        }
    }

    fn disassemble_instruction(&self) {
        if let Some(chunk) = &self.chunk {
            chunk.disassemble_instruction(self.ip);
        }
    }
}
