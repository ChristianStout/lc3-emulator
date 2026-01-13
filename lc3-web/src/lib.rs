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
    return asm.run(file);
}
