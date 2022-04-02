extern crate text_io;

mod chunk;
mod common;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

mod prelude {
    pub use crate::chunk::*;
    pub use crate::common::*;
    pub use crate::compiler::*;
    pub use crate::debug::*;
    pub use crate::scanner::*;
    pub use crate::value::*;
    pub use crate::vm::*;
}

use prelude::*;
use std::env;
use std::fs;
use std::io::{stdout, Write};
use std::process;
use text_io::read;

fn repl() {
    let mut vm = init_vm();
    loop {
        print!("> ");
        stdout().flush().expect("Failed to flush stdout.");
        let line: String = read!("{}\n");

        interpret(&mut vm, &line);
    }

    // free_vm(vm);
}

fn run_file(path: &str) {
    let mut vm = init_vm();
    let source = fs::read_to_string(path).expect("Couldn't read file.");
    let result = interpret(&mut vm, &source);
    free_vm(vm);

    match result {
        InterpretResult::Ok => process::exit(0),
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: lox-rs [path]");
        process::exit(64);
    }
}
