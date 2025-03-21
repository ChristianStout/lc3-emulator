use super::asm_ins::*;
use super::directive::*;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Tsify, Serialize,Deserialize)]
pub enum TokenType {
    Label(String),
    Instruction(OpcodeIns),
    Directive(Directive),
    Number(i16),
    String(String),
    Register(u16),
    INVALID(String),
}

#[derive(Debug, Clone, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Token {
    pub inner_token: TokenType,
    pub to: usize,
    pub from: usize,
    pub line_num: usize,
    pub original_match: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenCollection {
    pub tokens: Vec<Token>,
}

impl Token {
    pub fn new(index: usize, line_num: i32, string: &str, token: TokenType) -> Token {
        let original_match = string.to_string();
        let to = index - 1; // Because it only matches on the whitespace after the match
        let from = to - original_match.len();

        Token {
            inner_token: token,
            to: to - 1,
            from: from,
            line_num: line_num as usize,
            original_match: original_match,
        }
    }
    
    pub fn get_useless_token() -> Token {
        Token {
            inner_token: TokenType::INVALID("".to_string()),
            from: 0,
            to: 0,
            line_num: 0,
            original_match: "".to_string(),
        }
    }
}
