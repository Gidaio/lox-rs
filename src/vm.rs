use crate::prelude::*;

pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub fn init_vm() -> VM {
    VM {
        chunk: None,
        ip: 0,
        stack: vec![],
    }
}
pub fn free_vm(_vm: VM) {
    // Just take it and don't let it go.
}

pub fn interpret(vm: &mut VM, source: &str) -> InterpretResult {
    let mut chunk = init_chunk();

    if !compile(source, &mut chunk) {
        free_chunk(chunk);
        InterpretResult::CompileError
    } else {
        vm.chunk = Some(chunk);
        vm.ip = 0;

        run(vm)
    }
}

fn run(vm: &mut VM) -> InterpretResult {
    if let Some(ref mut chunk) = vm.chunk {
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("          ");
                for slot in &vm.stack {
                    print!("[ ");
                    print_value(*slot);
                    print!(" ]");
                }
                println!("");
                disassemble_instruction(chunk, vm.ip);
            }
            let instruction = chunk.code[vm.ip];
            vm.ip += 1;
            match instruction {
                OP_CONSTANT => {
                    let constant_index = chunk.code[vm.ip];
                    vm.ip += 1;
                    let constant = chunk.constants[constant_index as usize];
                    vm.stack.push(constant);
                }
                OP_ADD => {
                    let b = vm.stack.pop().expect("Tried to pop an empty stack.");
                    let a = vm.stack.pop().expect("Tried to pop an empty stack.");
                    vm.stack.push(a + b);
                }
                OP_SUBTRACT => {
                    let b = vm.stack.pop().expect("Tried to pop an empty stack.");
                    let a = vm.stack.pop().expect("Tried to pop an empty stack.");
                    vm.stack.push(a - b);
                }
                OP_MULTIPLY => {
                    let b = vm.stack.pop().expect("Tried to pop an empty stack.");
                    let a = vm.stack.pop().expect("Tried to pop an empty stack.");
                    vm.stack.push(a * b);
                }
                OP_DIVIDE => {
                    let b = vm.stack.pop().expect("Tried to pop an empty stack.");
                    let a = vm.stack.pop().expect("Tried to pop an empty stack.");
                    vm.stack.push(a / b);
                }
                OP_NEGATE => {
                    let value = vm.stack.pop().expect("Tried to pop an empty stack.");
                    vm.stack.push(-value);
                }
                OP_RETURN => {
                    print_value(vm.stack.pop().expect("Tried to pop an empty stack."));
                    println!("");
                    return InterpretResult::Ok;
                }
                _ => return InterpretResult::RuntimeError,
            }
        }
    } else {
        InterpretResult::CompileError
    }
}
