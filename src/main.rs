pub mod asm;
pub mod cli;
pub mod output;
pub mod vm;
pub mod web;

use crate::asm::asm::Asm;
use crate::vm::vm::VM;
// use std::env;
use std::fs;
use std::io::*;

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

    if cli.emit_bin {
        asm.emit_bin(&binary_file, "out.bin".to_string());
    }

    let mut vm = VM::new();
    vm.run(binary_file);
}
