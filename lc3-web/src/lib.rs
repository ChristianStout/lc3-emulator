pub mod highlight;
pub mod webio;
pub mod webvm;
use lc3::asm::asm::Asm;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

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
