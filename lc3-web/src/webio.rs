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
    
    fn put_char(&mut self, c: char) {
        // let console = document.getElementById("innerConsole");
        // if c == {DELETE KEY} {
        //   console.value = console.value[:-1];
        //   return;
        // }
        // console.value += c;
    }
}
