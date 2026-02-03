pub mod highlight;
pub mod webio;
pub mod webvm;
use lc3::asm::asm::Asm;
use lc3::asm::lexer::*;
use lc3::asm::token::*;
use lc3::io::Lc3IO;
use lc3::vm::vm::OPCODE_DELTA;
use wasm_bindgen::prelude::*;

use crate::webio::WebIO;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn assemble(file: String) -> Option<Vec<u16>> {
    let io = Lc3IO::new(Box::new(WebIO::new()));
    let mut asm = Asm::new();
    return asm.run(file);
}

#[wasm_bindgen]
pub fn get_tokens(file: String) -> TokenCollection {
    return TokenCollection {
        tokens: Lexer::new().run(file),
    };
}

#[wasm_bindgen]
pub fn u16_to_ascii_rep(n: u16) -> String {
    match n {
        0 => "NUL".to_string(),
        1 => "SOH".to_string(),
        2 => "STX".to_string(),
        3 => "ETX".to_string(),
        4 => "EOT".to_string(),
        5 => "ENQ".to_string(),
        6 => "ACK".to_string(),
        7 => "BEL".to_string(),
        8 => "BS".to_string(),
        9 => "TAB".to_string(),
        10 => "NEW LINE".to_string(),
        11 => "VT".to_string(),
        12 => "FF".to_string(),
        13 => "CR".to_string(),
        14 => "SO".to_string(),
        15 => "SI".to_string(),
        16 => "DLE".to_string(),
        17 => "DC1".to_string(),
        18 => "DC2".to_string(),
        19 => "DC3".to_string(),
        20 => "DC4".to_string(),
        21 => "NAK".to_string(),
        22 => "SYN".to_string(),
        23 => "ETB".to_string(),
        24 => "CAN".to_string(),
        25 => "EM".to_string(),
        26 => "SUB".to_string(),
        27 => "ESC".to_string(),
        28 => "FS".to_string(),
        29 => "GS".to_string(),
        30 => "RS".to_string(),
        31 => "US".to_string(),
        32 => "SPACE".to_string(),
        127 => "DEL".to_string(),
        _ => String::from(n as u8 as char),
    }
}

#[wasm_bindgen]
pub fn u16_to_instr_rep(n: u16) -> String {
    if n == 0 {
        return "NO-OP".to_string();
    }
    let opcode = n >> OPCODE_DELTA;

    match opcode {
        0 => "BR".to_string(),
        1 => "ADD".to_string(),
        2 => "LD".to_string(),
        3 => "ST".to_string(),
        4 => {
            let without_op = n - (opcode << OPCODE_DELTA);
            let code = without_op >> OPCODE_DELTA - 1;
            if code > 0 {
                "JSR".to_string()
            } else {
                "JSRR".to_string()
            }
        }
        5 => "AND".to_string(),
        6 => "LDR".to_string(),
        7 => "STR".to_string(),
        8 => "RTI".to_string(),
        9 => "NOT".to_string(),
        10 => "LDI".to_string(),
        11 => "STI".to_string(),
        12 => "JMP".to_string(),
        13 => "RESERVED".to_string(),
        14 => "LEA".to_string(),
        15 => {
            let without_op = n - (opcode << OPCODE_DELTA);
            get_trap_string(without_op)
        }
        _ => "-".to_string(),
    }
}

fn get_trap_string(without_op: u16) -> String {
    match without_op {
        0x20 => "GETC".to_string(),
        0x21 => "OUT".to_string(),
        0x22 => "PUTS".to_string(),
        0x23 => "IN".to_string(),
        0x24 => "PUTSP".to_string(),
        0x25 => "HALT".to_string(),
        _ => "TRAP".to_string(),
    }
}
