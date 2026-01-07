use crate::vm::vm::VM;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_vm() -> VM {
    return VM::new();
}

// #[wasm_bindgen]
// pub fn
