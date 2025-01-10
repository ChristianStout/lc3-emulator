use super::registers::Registers;
use super::memory::Memory;

/*
Uses the command pattern to execute functions dynamically
*/

pub trait Instruction {
    /*
    value is the raw instruction interpretted from the asm,
    *excluding* the opcode.
    */
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory);
}

#[allow(dead_code, unused_variables)]
pub struct Add;
pub struct And;
pub struct Br;
pub struct JmpRet;
pub struct Jsr;
pub struct Ld;
pub struct Ldi;
pub struct Ldr;
pub struct Lea;
pub struct Not;
pub struct Rti;
pub struct St;
pub struct Sti;
pub struct Str;
pub struct Trap;

impl Instruction for Add {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {
        /*
        ADD - | 0001 000 000 000 000 |
              | ---- --- --- --- --- |
              | op   dr  sr1 --- sr2 |
        */
        let mut i = value;

        let dr = i << 9;
        i -= dr >> 9;

        let sr1 = i << 6;
        i -= sr1 >> 6;

        let sr2 = i;

        let v1 = reg.get(sr1 as usize);
        let v2 = reg.get(sr2 as usize);

        reg.set(dr as usize, v1 + v2);
    }
}

impl Instruction for And {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {
        /*
        AND - | 0101 000 000 000 000 |
              | ---- --- --- --- --- |
              | op   dr  sr1 --- sr2 |
              +----------------------+
        AND - | 0101 000 000 1 00000 |
              | ---- --- --- - ----- |
              | op   dr  sr1 - imm   |
        */
        let mut i = value;

        let dr = i << 9;
        i -= dr >> 9;

        let sr1 = i << 6;
        i -= dr >> 6;

        let code = i << 5;
        match code {
            0 => {
                let sr2 = i;

                let v1 = reg.get(sr1 as usize);
                let v2 = reg.get(sr2 as usize);

                reg.set(dr as usize, v1 & v2);
            },
            1 => {
                i -= code >> 5;
                let reg_val = reg.get(sr1 as usize);
                let imm_val = i;
                
                reg.set(dr as usize, reg_val & imm_val);
            },
            _ => {
                unreachable!();
            },
        }
    }
}

impl Instruction for Br {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for JmpRet {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Jsr {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Ld {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Ldi {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Ldr {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Lea {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Not {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Rti {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for St {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Sti {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Str {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}

impl Instruction for Trap {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {

    }
}
