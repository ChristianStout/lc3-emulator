/* This file is essential for wasm to be able to use the
 * &mut VM functions available in VM. Otherwise, type safetyu is violated.
 */
use super::webio::WebIO;
use lc3::io::Lc3IO;
use lc3::vm::vm::InstructionResult;
use lc3::vm::vm::VM;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct WebVM {
    vm: VM,
    awaiting_input: bool,
}

#[allow(dead_code)]
#[wasm_bindgen]
impl WebVM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebVM {
        WebVM {
            vm: VM::new(Lc3IO::new(Box::new(WebIO::new()))),
            awaiting_input: false,
        }
    }

    pub async fn step(&mut self) -> Result<(), JsValue> {
        let result = self.vm.run_single_command();
        match result {
            InstructionResult::AwaitingInput => {
                self.awaiting_input = true;
            }
            _ => {
                self.awaiting_input = false;
            }
        }
        return Ok(());
    }

    pub fn load_into_memory(&mut self, file: Vec<u16>) {
        self.vm.memory.load_file(file);
    }

    pub async fn reset_machine(&mut self) {
        self.vm.memory.clear();
        self.vm.registers.halt = false;
        self.awaiting_input = false;
    }

    pub fn set_pc(&mut self, new_pc: u16) {
        self.vm.registers.pc = new_pc;
    }

    pub fn get_pc(&self) -> u16 {
        return self.vm.registers.pc;
    }

    pub fn is_halted(&mut self) -> bool {
        return self.vm.registers.halt;
    }

    pub async fn set_reg(&mut self, reg: usize, value: u16) {
        self.vm.registers.set(reg, value);
    }

    pub async fn get_reg_value_as_hex(&self, reg_value: usize) -> String {
        let value = self.vm.registers.get(reg_value);
        return format!("x{:04X}", value);
    }

    pub async fn get_pc_value_as_hex(&self) -> String {
        let value = self.vm.registers.pc;
        return format!("x{:04X}", value);
    }

    pub async fn get_ir_value_as_hex(&self) -> String {
        let value = self.vm.registers.ir;
        return format!("x{:04X}", value);
    }

    pub async fn is_awaiting_input(&self) -> bool {
        return self.awaiting_input;
    }

    pub async fn set_awaiting_input(&mut self, is: bool) {
        self.awaiting_input = is;
    }
}
