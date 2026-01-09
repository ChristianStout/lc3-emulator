pub mod asm;
pub mod io;
pub mod vm;
use crate::asm::lexer::*;
use crate::asm::token::*;
#[cfg(feature = "serde")]
use wasm_bindgen::prelude::*;

// // Import the `window.alert` function from the Web.
// #[cfg_attr(feature = "serde", wasm_bindgen)]
// extern "C" {
//     fn alert(s: &str);
// }

// // Export a `greet` function from Rust to JavaScript, that alerts a
// // hello message.
// #[cfg_attr(feature = "serde", wasm_bindgen)]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

#[cfg_attr(feature = "serde", wasm_bindgen)]
pub fn get_tokens(file: String) -> TokenCollection {
    return TokenCollection {
        tokens: Lexer::new().run(file),
    };
}
