use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

pub const GETC_VAL: u16 = 0x20;
pub const OUT_VAL: u16 = 0x21;
pub const PUTS_VAL: u16 = 0x22;
pub const IN_VAL: u16 = 0x23;
pub const HALT_VAL: u16 = 0x25;

pub enum OperandType {
    /*
    # OperandType
    This enum refers to the types
    of operands an instruction could have
    */
    Reg,
    Label,
    Imm,
    RegOrImm,
    String,
}

#[derive(Debug, PartialEq, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[allow(dead_code)]
pub enum OpcodeIns {
    Add,
    And,
    Br(bool, bool, bool),
    Jmp,
    Jsr,
    Jsrr,
    Ld,
    Ldi,
    Ldr,
    Lea,
    Not,
    Ret,
    Rti,
    St,
    Sti,
    Str,
    Trap(u16),
    Reserved,
    INVALID,
}

impl OperandType {
    pub fn as_string(&self) -> String {
        match self {
            OperandType::Reg => "a register".to_string(),
            OperandType::Label => "a label".to_string(),
            OperandType::Imm => "an immediate value".to_string(),
            OperandType::RegOrImm => "a register or immediate value".to_string(),
            OperandType::String => "a string".to_string(),
        }
    }
}

#[allow(dead_code)]
impl OpcodeIns {
    pub fn from(name: &str) -> OpcodeIns {
        let upper_name: &str = &name.to_uppercase();

        if &upper_name[..2] == "BR" {
            return OpcodeIns::get_br(&upper_name[2..]);
        }

        match upper_name {
            "ADD" => return OpcodeIns::Add,
            "AND" => return OpcodeIns::And,
            "JMP" => return OpcodeIns::Jmp,
            "JSR" => return OpcodeIns::Jsr,
            "JSRR" => return OpcodeIns::Jsrr,
            "LD" => return OpcodeIns::Ld,
            "LDI" => return OpcodeIns::Ldi,
            "LDR" => return OpcodeIns::Ldr,
            "LEA" => return OpcodeIns::Lea,
            "NOT" => return OpcodeIns::Not,
            "RET" => return OpcodeIns::Ret,
            "RTI" => return OpcodeIns::Rti,
            "ST" => return OpcodeIns::St,
            "STI" => return OpcodeIns::Sti,
            "STR" => return OpcodeIns::Str,
            "GETC" => return OpcodeIns::Trap(GETC_VAL),
            "OUT" => return OpcodeIns::Trap(OUT_VAL),
            "PUTS" => return OpcodeIns::Trap(PUTS_VAL),
            "IN" => return OpcodeIns::Trap(IN_VAL),
            "HALT" => return OpcodeIns::Trap(HALT_VAL),
            _ => return OpcodeIns::INVALID,
        }
    }

    pub fn get_expected_operands(&self) -> VecDeque<OperandType> {
        match self {
            OpcodeIns::Add => vec![OperandType::Reg, OperandType::Reg, OperandType::RegOrImm]
                .into_iter()
                .collect(),
            OpcodeIns::And => vec![OperandType::Reg, OperandType::Reg, OperandType::RegOrImm]
                .into_iter()
                .collect(),
            OpcodeIns::Br(_, _, _) => vec![OperandType::Label].into_iter().collect(),
            OpcodeIns::Jmp => vec![OperandType::Reg].into_iter().collect(),
            OpcodeIns::Jsr => vec![OperandType::Label].into_iter().collect(),
            OpcodeIns::Jsrr => vec![OperandType::Reg].into_iter().collect(),
            OpcodeIns::Ld => vec![OperandType::Reg, OperandType::Label]
                .into_iter()
                .collect(),
            OpcodeIns::Ldi => vec![OperandType::Reg, OperandType::Label]
                .into_iter()
                .collect(),
            OpcodeIns::Ldr => vec![OperandType::Reg, OperandType::Reg, OperandType::Imm]
                .into_iter()
                .collect(),
            OpcodeIns::Lea => vec![OperandType::Reg, OperandType::Label]
                .into_iter()
                .collect(),
            OpcodeIns::Not => vec![OperandType::Reg, OperandType::Reg]
                .into_iter()
                .collect(),
            OpcodeIns::Ret => vec![].into_iter().collect(),
            OpcodeIns::Rti => vec![].into_iter().collect(),
            OpcodeIns::St => vec![OperandType::Reg, OperandType::Label]
                .into_iter()
                .collect(),
            OpcodeIns::Sti => vec![OperandType::Reg, OperandType::Label]
                .into_iter()
                .collect(),
            OpcodeIns::Str => vec![OperandType::Reg, OperandType::Reg, OperandType::Imm]
                .into_iter()
                .collect(),
            OpcodeIns::Trap(subroutine) => self.get_expected_operand_for_trap(*subroutine),
            _ => vec![].into_iter().collect(),
        }
    }

    fn get_expected_operand_for_trap(&self, subroutine: u16) -> VecDeque<OperandType> {
        match subroutine {
            0x20 => vec![].into_iter().collect(),
            0x21 => vec![].into_iter().collect(),
            0x22 => vec![].into_iter().collect(),
            0x23 => vec![].into_iter().collect(),
            0x25 => vec![].into_iter().collect(),
            _ => {
                panic!(
                    "asm_ins::OpcodeIns::get_expected_operand_for_trap(): Received an impossible trap subroutine number."
                );
            }
        }
    }

    fn get_br(nzp: &str) -> OpcodeIns {
        // nzp only contained everything AFTER br, that being nzp

        let mut n = false;
        let mut z = false;
        let mut p = false;

        for c in nzp.chars() {
            match c {
                'N' => {
                    if n {
                        return OpcodeIns::INVALID;
                    }
                    n = true;
                }
                'Z' => {
                    if z {
                        return OpcodeIns::INVALID;
                    }
                    z = true;
                }
                'P' => {
                    if p {
                        return OpcodeIns::INVALID;
                    }
                    p = true;
                }
                _ => return OpcodeIns::INVALID,
            }
        }
        /*
        if !n && !z && !p {
            return OpcodeIns::Br(true, true, true);
        } */

        if !n && !z && !p {
            n = true;
            z = true;
            p = true;
        }

        return OpcodeIns::Br(n, z, p);
    }

    pub fn get_immediate_value_width(&self) -> Option<i32> {
        match self {
            OpcodeIns::Add | OpcodeIns::And => Some(5),
            OpcodeIns::Ldr | OpcodeIns::Str => Some(6),
            OpcodeIns::Br(_, _, _) | OpcodeIns::Ld | OpcodeIns::Ldi => Some(9),
            OpcodeIns::Lea | OpcodeIns::St | OpcodeIns::Sti => Some(9),
            OpcodeIns::Jsr => Some(11),
            _ => None,
        }
    }

    pub fn get_opcode_value(&self) -> u16 {
        match self {
            OpcodeIns::Br(_, _, _) => 0,
            OpcodeIns::Add => 1,
            OpcodeIns::Ld => 2,
            OpcodeIns::St => 3,
            OpcodeIns::Jsr | OpcodeIns::Jsrr => 4,
            OpcodeIns::And => 5,
            OpcodeIns::Ldr => 6,
            OpcodeIns::Str => 7,
            OpcodeIns::Rti => 8,
            OpcodeIns::Not => 9,
            OpcodeIns::Ldi => 10,
            OpcodeIns::Sti => 11,
            OpcodeIns::Jmp | OpcodeIns::Ret => 12,
            OpcodeIns::Lea => 14,
            OpcodeIns::Trap(_) => 15,
            // OpcodeIns::Reserved => 13,
            OpcodeIns::Reserved | OpcodeIns::INVALID => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::asm::asm_ins::*;

    #[test]
    fn test_get_ins_from_str() {
        assert!(OpcodeIns::from("ADD") == OpcodeIns::Add);
        assert!(OpcodeIns::from("AND") == OpcodeIns::And);
        assert!(OpcodeIns::from("BR") == OpcodeIns::Br(true, true, true));
        assert!(OpcodeIns::from("BRnzp") == OpcodeIns::Br(true, true, true));
        assert!(OpcodeIns::from("JMP") == OpcodeIns::Jmp);
        assert!(OpcodeIns::from("JSR") == OpcodeIns::Jsr);
        assert!(OpcodeIns::from("JSRR") == OpcodeIns::Jsrr);
        assert!(OpcodeIns::from("LD") == OpcodeIns::Ld);
        assert!(OpcodeIns::from("LDI") == OpcodeIns::Ldi);
        assert!(OpcodeIns::from("LDR") == OpcodeIns::Ldr);
        assert!(OpcodeIns::from("LEA") == OpcodeIns::Lea);
        assert!(OpcodeIns::from("NOT") == OpcodeIns::Not);
        assert!(OpcodeIns::from("RET") == OpcodeIns::Ret);
        assert!(OpcodeIns::from("RTI") == OpcodeIns::Rti);
        assert!(OpcodeIns::from("ST") == OpcodeIns::St);
        assert!(OpcodeIns::from("STI") == OpcodeIns::Sti);
        assert!(OpcodeIns::from("STR") == OpcodeIns::Str);
        assert!(OpcodeIns::from("GETC") == OpcodeIns::Trap(GETC_VAL));
        assert!(OpcodeIns::from("OUT") == OpcodeIns::Trap(OUT_VAL));
        assert!(OpcodeIns::from("PUTS") == OpcodeIns::Trap(PUTS_VAL));
        assert!(OpcodeIns::from("IN") == OpcodeIns::Trap(IN_VAL));
        assert!(OpcodeIns::from("HALT") == OpcodeIns::Trap(HALT_VAL));

        assert!(OpcodeIns::from("HALTT") == OpcodeIns::INVALID);
        assert!(OpcodeIns::from("LLEA") == OpcodeIns::INVALID);
        assert!(OpcodeIns::from("LEAA") == OpcodeIns::INVALID);
        assert!(OpcodeIns::from("LEEA") == OpcodeIns::INVALID);
        assert!(OpcodeIns::from("WHAT") == OpcodeIns::INVALID);
        assert!(OpcodeIns::from("VAR") == OpcodeIns::INVALID);
        assert!(OpcodeIns::from("OKAYTHEN") == OpcodeIns::INVALID);
        assert!(OpcodeIns::from("WHATAMIEVENDOINGRIGHTNOW???") == OpcodeIns::INVALID);
    }

    #[test]
    fn test_get_ins_from_str_case_sensitivity() {
        assert!(OpcodeIns::from("add") == OpcodeIns::Add);
        assert!(OpcodeIns::from("Add") == OpcodeIns::Add);
        assert!(OpcodeIns::from("ADd") == OpcodeIns::Add);
        assert!(OpcodeIns::from("AdD") == OpcodeIns::Add);
        assert!(OpcodeIns::from("aDd") == OpcodeIns::Add);
        assert!(OpcodeIns::from("aDD") == OpcodeIns::Add);
        assert!(OpcodeIns::from("adD") == OpcodeIns::Add);
        assert!(OpcodeIns::from("ADD") == OpcodeIns::Add);

        assert!(OpcodeIns::from("and") == OpcodeIns::And);
        assert!(OpcodeIns::from("aNd") == OpcodeIns::And);
        assert!(OpcodeIns::from("AND") == OpcodeIns::And);

        assert!(OpcodeIns::from("not") == OpcodeIns::Not);
        assert!(OpcodeIns::from("NOT") == OpcodeIns::Not);
        assert!(OpcodeIns::from("LD") == OpcodeIns::Ld);
        assert!(OpcodeIns::from("lD") == OpcodeIns::Ld);
        assert!(OpcodeIns::from("hAlT") == OpcodeIns::Trap(HALT_VAL));
        assert!(OpcodeIns::from("halt") == OpcodeIns::Trap(HALT_VAL));
    }

    #[test]
    fn test_get_br() {
        assert!(OpcodeIns::get_br("") == OpcodeIns::Br(true, true, true));
        assert!(OpcodeIns::get_br("N") == OpcodeIns::Br(true, false, false));
        assert!(OpcodeIns::get_br("NZ") == OpcodeIns::Br(true, true, false));
        assert!(OpcodeIns::get_br("NP") == OpcodeIns::Br(true, false, true));
        assert!(OpcodeIns::get_br("Z") == OpcodeIns::Br(false, true, false));
        assert!(OpcodeIns::get_br("ZP") == OpcodeIns::Br(false, true, true));
        assert!(OpcodeIns::get_br("NZP") == OpcodeIns::Br(true, true, true));

        assert!(OpcodeIns::get_br("NN") == OpcodeIns::INVALID);
        assert!(OpcodeIns::get_br("PP") == OpcodeIns::INVALID);
        assert!(OpcodeIns::get_br("ZZ") == OpcodeIns::INVALID);

        assert!(OpcodeIns::get_br("NZZP") == OpcodeIns::INVALID);
        assert!(OpcodeIns::get_br("NNZZPP") == OpcodeIns::INVALID);
        assert!(OpcodeIns::get_br("M") == OpcodeIns::INVALID);
        assert!(OpcodeIns::get_br("PR") == OpcodeIns::INVALID);
        assert!(OpcodeIns::get_br("?") == OpcodeIns::INVALID);
        assert!(OpcodeIns::get_br("ADD") == OpcodeIns::INVALID);
        assert!(OpcodeIns::get_br("okay") == OpcodeIns::INVALID);
    }
}
