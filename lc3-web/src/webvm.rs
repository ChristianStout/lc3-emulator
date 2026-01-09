/* This file is essential for wasm to be able to use the
 * &mut VM functions available in VM. Otherwise, type safetyu is violated.
 */
use super::webio::WebIO;
use lc3::io::Lc3IO;
use lc3::vm::vm::VM;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct WebVM {
    vm: VM,
}

#[wasm_bindgen]
impl WebVM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebVM {
        WebVM {
            vm: VM::new(Lc3IO::new(Box::new(WebIO::new()))),
        }
    }

    pub fn run(&mut self, file: Vec<u16>) {
        self.vm.run(file);
    }

    pub fn step(&mut self) {
        self.vm.run_single_command();
    }

    pub fn load_into_memory(&mut self, file: Vec<u16>) {
        self.vm.memory.load_file(file);
    }

    pub fn clear_memory(&mut self) {
        self.vm.memory.clear();
    }

    pub fn set_pc(&mut self, new_pc: u16) {
        self.vm.registers.pc = new_pc;
    }

    pub fn is_halted(&mut self) -> bool {
        return self.vm.registers.halt;
    }
}
