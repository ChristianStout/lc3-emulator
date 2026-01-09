use super::vm::memory::Memory;
use super::vm::registers::Registers;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
#[cfg(feature = "serde")]
use tsify::Tsify;
#[cfg(feature = "serde")]
use typetag;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize, Tsify))]
#[cfg_attr(feature = "serde", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Lc3IO {
    input_stream: VecDeque<char>,
    output_stream: String,
    target: Box<dyn IOTarget>,
}

impl Lc3IO {
    pub fn new(target: Box<dyn IOTarget>) -> Lc3IO {
        Lc3IO {
            input_stream: VecDeque::new(),
            output_stream: String::new(),
            target: target,
        }
    }

    pub fn get_char(&mut self) -> char {
        return self.target.get_char();
    }

    pub fn print_string(&mut self, reg: &mut Registers, mem: &mut Memory) {
        self.target.print_string(reg, mem);
    }

    pub fn print_string_special(&mut self, reg: &mut Registers, mem: &mut Memory) {
        self.target.print_string_special(reg, mem);
    }

    pub fn print_single_char(&mut self, reg: &mut Registers) {
        self.target.print_single_char(reg);
    }

    pub fn print_vm_error(&mut self, error_name: &str, error_msg: &str) {
        self.target.print_vm_error(error_name, error_msg);
    }

    pub fn print_asm_error(&mut self, error: &str) {
        self.target.print_asm_error(error);
    }
}

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait IOTarget {
    fn get_char(&mut self) -> char;
    fn print_string(&mut self, reg: &mut Registers, mem: &mut Memory);
    fn print_string_special(&mut self, reg: &mut Registers, mem: &mut Memory);
    fn print_single_char(&mut self, reg: &mut Registers);
    fn print_asm_error(&mut self, err_msg: &str);
    fn print_vm_error(&mut self, error_name: &str, error_msg: &str);
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize, Tsify))]
#[cfg_attr(feature = "serde", tsify(into_wasm_abi, from_wasm_abi))]
pub struct DebugIO {
    input_stream: VecDeque<u8>,
    output_stream: VecDeque<u8>,
}

impl DebugIO {
    pub fn new() -> DebugIO {
        DebugIO {
            input_stream: VecDeque::new(),
            output_stream: VecDeque::new(),
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl IOTarget for DebugIO {
    fn get_char(&mut self) -> char {
        return '0';
    }
    fn print_string(&mut self, _reg: &mut Registers, _mem: &mut Memory) {}
    fn print_string_special(&mut self, _reg: &mut Registers, _mem: &mut Memory) {}
    fn print_single_char(&mut self, _reg: &mut Registers) {}
    fn print_asm_error(&mut self, _err_msg: &str) {}
    fn print_vm_error(&mut self, _error_name: &str, _error_msg: &str) {}
}
