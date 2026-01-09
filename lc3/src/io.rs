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
    target: Box<dyn IOTarget>,
}

impl Lc3IO {
    pub fn new(target: Box<dyn IOTarget>) -> Lc3IO {
        Lc3IO {
            target: target,
        }
    }

    pub fn get_char(&mut self) -> char {
        return self.target.get_char();
    }

    pub fn put_char(&mut self, c: char) {
        self.target.put_char(c);
    }

    pub fn print_error(&mut self, error: String) {
        for c in error.chars() {
            self.target.put_char(c);
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait IOTarget {
    fn get_char(&mut self) -> char;
    fn put_char(&mut self, c: char);
    // fn print_string(&mut self, reg: &mut Registers, mem: &mut Memory);
    // fn print_string_special(&mut self, reg: &mut Registers, mem: &mut Memory);
    // fn print_single_char(&mut self, reg: &mut Registers);
    // fn print_asm_error(&mut self, err_msg: &str);
    // fn print_vm_error(&mut self, error_name: &str, error_msg: &str);
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize, Tsify))]
#[cfg_attr(feature = "serde", tsify(into_wasm_abi, from_wasm_abi))]
pub struct DebugIO {
    input_stream: VecDeque<char>,
    output_stream: VecDeque<char>,
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
        return self.input_stream.pop_front().expect("You forgot to put something in DebugIO input stream.")
    }
    fn put_char(&mut self, c: char) {
        self.output_stream.push_back(c);
    }
    // fn print_string(&mut self, _reg: &mut Registers, _mem: &mut Memory) {}
    // fn print_string_special(&mut self, _reg: &mut Registers, _mem: &mut Memory) {}
    // fn print_single_char(&mut self, _reg: &mut Registers) {}
    // fn print_asm_error(&mut self, _err_msg: &str) {}
    // fn print_vm_error(&mut self, _error_name: &str, _error_msg: &str) {}
}
