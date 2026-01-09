use std::collections::VecDeque;

use lc3::io::IOTarget;
use lc3::vm::memory::Memory;
use lc3::vm::registers::Registers;
use serde::Deserialize;
use serde::Serialize;
use tsify::Tsify;
use typetag;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WebIO {
    output_stream: VecDeque<char>,
    input_stream: VecDeque<char>,
}

impl WebIO {
    pub fn new() -> WebIO {
        WebIO {
            output_stream: VecDeque::new(),
            input_stream: VecDeque::new(),
        }
    }
}

#[typetag::serde]
impl IOTarget for WebIO {
    fn get_char(&mut self) -> char {
        while self.input_stream.len() == 0 {}
        return self
            .input_stream
            .pop_front()
            .expect("Should not been able to pop from the input stream while empty");
    }
    fn print_string(&mut self, reg: &mut Registers, mem: &mut Memory) {
        let mut i = reg.get(0);
        let mut c = mem.get(i) as u8 as char;

        while c != '\0' {
            self.output_stream.push_back(c);
            i += 1;
            c = mem.get(i) as u8 as char;
        }
    }
    fn print_string_special(&mut self, _reg: &mut Registers, _mem: &mut Memory) {}
    fn print_single_char(&mut self, reg: &mut Registers) {
        self.output_stream.push_back(reg.get(0) as u8 as char);
    }

    fn print_vm_error(&mut self, error_name: &str, error_msg: &str) {
        let msg = format!("{error_name}: {error_msg}\n");
        for c in msg.chars() {
            self.output_stream.push_back(c);
        }
    }

    fn print_asm_error(&mut self, err_msg: &str) {
        let msg = format!("{}", err_msg);
        for c in msg.chars() {
            self.output_stream.push_back(c);
        }
    }
}
