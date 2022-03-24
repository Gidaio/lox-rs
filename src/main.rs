mod chunk;
mod compiler;
mod debug;
mod opcode;
mod scanner;
mod value;
mod vm;

use std::io::{Read, Write};
use std::process::exit;
use text_io::read;
use vm::{InterpretResult, VM};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        println!("Usage: clox [path]");
        exit(64);
    }
}

fn repl() {
    // let mut vm = VM::init();

    loop {
        print!("> ");
        std::io::stdout()
            .flush()
            .expect("Woah. Somehow flushing stdout failed.");
        let input: String = read!("{}\n");
        VM::interpret(input);
    }
}

fn run_file(path: &str) {
    let source = read_file(path);
    // let mut vm = VM::init();
    let result = VM::interpret(source);

    match result {
        InterpretResult::Ok => (),
        InterpretResult::CompileError => exit(65),
        InterpretResult::RuntimeError => exit(70),
    }
}

fn read_file(path: &str) -> String {
    let mut source = String::new();
    let mut file = if let Ok(file) = std::fs::File::open(path) {
        file
    } else {
        println!("Couldn't open file \"{}\".", path);
        exit(74);
    };
    if let Err(_) = file.read_to_string(&mut source) {
        println!("Hey! The file you gave me isn't text!");
        exit(74);
    };

    source
}
