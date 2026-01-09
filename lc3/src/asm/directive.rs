use super::asm_ins::OperandType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
#[cfg(feature = "serde")]
use tsify::Tsify;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Tsify, Serialize, Deserialize))]
#[cfg_attr(feature = "serde", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Directive {
    ORIG,
    FILL,
    BLKW,
    STRINGZ,
    END,
}

impl Directive {
    pub fn from(word: &str) -> Directive {
        match word {
            ".ORIG" => return Directive::ORIG,
            ".FILL" => return Directive::FILL,
            ".BLKW" => return Directive::BLKW,
            ".STRINGZ" => return Directive::STRINGZ,
            ".END" => return Directive::END,
            _ => unreachable!(),
        }
    }

    pub fn get_expected_operands(&self) -> VecDeque<OperandType> {
        match self {
            Directive::ORIG | Directive::FILL | Directive::BLKW => {
                vec![OperandType::Imm].into_iter().collect()
            }
            Directive::STRINGZ => vec![OperandType::String].into_iter().collect(),
            _ => vec![].into_iter().collect(),
        }
    }
}
