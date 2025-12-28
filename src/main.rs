pub mod asm;
pub mod cli;
pub mod output;
pub mod vm;
pub mod web;

use crate::asm::asm::Asm;
use crate::vm::vm::VM;
// use std::env;
use std::fs;

fn main() {
    let cli = cli::get_cli();

    let file_path = cli.file_path;
    let file;
    if let Ok(path) = fs::read_to_string(file_path) {
        file = path;
    }
    // FOR DEBUGGING PURPOSES ONLY
    else {
        file = "test.asm".to_string();
    }

    let mut asm = Asm::new();
<<<<<<< Updated upstream
    let binary_file: Vec<u16>;
    if let Some(out) = asm.run(file) {
        binary_file = out;
    } else {
        return;
    }
    
=======
    let binary_file = asm.run(file);

>>>>>>> Stashed changes
    let mut vm = VM::new();
    vm.run(binary_file);
}
