mod highlight;
mod webio;
use lc3::{asm::asm::Asm, vm::vm::VM};
use wasm_bindgen::prelude::*;
use webio::WebIO;

#[wasm_bindgen]
pub fn assemble(file: String) -> Option<Vec<u16>> {
    let mut asm = Asm::new();
    let binary_file: Vec<u16>;
    if let Some(out) = asm.run(file) {
        binary_file = out;
    } else {
        return None;
    }

    return Some(binary_file);
}

#[wasm_bindgen]
pub fn get_vm() -> VM {
    return VM::new();
}

#[wasm_bindgen]
pub fn get_io() -> WebIO {
    return WebIO::new();
}
