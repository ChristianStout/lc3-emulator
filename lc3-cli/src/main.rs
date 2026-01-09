mod cli;
mod stdio;

use lc3;
use lc3::asm::asm::Asm;
use lc3::io::Lc3IO;
use lc3::vm::vm::VM;
use std::fs;
use stdio::*;

fn main() {
    let cli = cli::get_cli();

    let file_path_input = cli.file_path;
    let file;
    if let Ok(path) = fs::read_to_string(file_path_input) {
        file = path;
    } else {
        println!("The asm file provided does not exist.");
        return;
    }

    let mut asm = Asm::new();
    let binary_file: Vec<u16>;
    if let Some(out) = asm.run(file) {
        binary_file = out;
    } else {
        return;
    }

    if cli.emit_binary {
        asm.emit_bin(&binary_file, "out.bin".to_string());
    }

    let io = Lc3IO::new(Box::new(StdIOTarget {}));

    let mut vm = VM::new(io);
    vm.run(binary_file);
}
