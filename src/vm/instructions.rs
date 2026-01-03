use crate::io::Lc3IO;
use super::memory::Memory;
use super::registers::Registers;
use super::trap::Trap;
use crate::asm::asm_ins::{GETC_VAL, HALT_VAL, IN_VAL, OUT_VAL, PUTS_VAL};

/*
Uses the command pattern to execute functions dynamically
*/

#[allow(dead_code)]
pub trait Instruction {
    /*
    value is the raw instruction interpreted from the asm,
    *excluding* the opcode.
     ^^^^^^^^^
    This is because we already had to obtain that information
    in order to dynamically call the correct instruction.
    */
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory, io: &mut Lc3IO);
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

impl Instruction for Add {
    /// Add adds two numbers together, and stores it in a separate register. If the control code (6th from the end)
    /// is 0, then the last three bits are the second source register. If the control
    /// bit is 1, then the last 5 bits are a 2's complement immediate value.
    ///
    /// The result of the operation modifies the nzp bits depending on the outcome.
    /// If the result is negative, then n == true, z, p == false.
    /// If the result is zero, te z == true, n, p == false.
    /// If the result is positive, then p == true, n, z == false
    ///
    ///        AND - | 0101 000 000 000 000  |
    ///              | ---- --- --- - -- --- |
    ///              | op   dr  sr1 c -- sr2 |
    ///              +-----------------------+
    ///        AND - | 0101 000 000 1 00000  |
    ///              | ---- --- --- - -----  |
    ///              | op   dr  sr1 c imm5   |
    ///
    /// Legend:
    ///     op ---> opcode,
    ///     dr ---> desitination register
    ///     sr1 --> source register 1
    ///     sr2 --> source register 2
    ///     imm5 -> immediate value (5-bit 2's compliement)
    ///
    /// Example:
    ///
    ///     ADD R0, R1, R2 ; -> memory[R0] = memory[R1] + memory[R2]
    ///
    ///     ADD R0, R1, #2 ; -> memory[R0] = memory[R1] + 2
    ///
    /// **NOTE**:
    /// The immeidate value is only a 5-bit 2's complement number. Therefore the range accepted as
    /// an immediate value is [-16, 15].
    /// -------------------------------------------------------------------------------
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory, _io: &mut Lc3IO) {
        let mut i = value;

        let dr = i >> 9;
        i -= dr << 9;

        let sr1 = i >> 6;
        i -= sr1 << 6;

        let new_value: u16;
        let code = get_bit_index(value, 5);

        match code {
            0 => {
                let sr2 = i;

                let v1 = reg.get(sr1 as usize);
                let v2 = reg.get(sr2 as usize);

                new_value = (v1 as i16 + v2 as i16) as u16;
            }
            1 => {
                let reg_val = reg.get(sr1 as usize);
                let imm_val = get_offset(value, 5); // TODO: rename get_offset to something more useful, like get_last_bits
                new_value = (reg_val as i16 + imm_val as i16) as u16;
            }
            _ => unreachable!(),
        }

        reg.set(dr as usize, new_value);

        set_nzp(reg, new_value);
    }
}

impl Instruction for And {
    /// And does a bitwise `&` operation. If the control code (6th from the end)
    /// is 0, then the last three bits are the second source register. If the control
    /// bit is 1, then the last 5 bits are a 2's complement immediate value.
    ///
    /// The result of the operation modifies the nzp bits depending on the outcome.
    /// If the result is negative, then n == true, z, p == false.
    /// If the result is zero, te z == true, n, p == false.
    /// If the result is positive, then p == true, n, z == false
    ///
    /// # Instruction Layout
    ///
    ///        AND - | 0101 000 000 0 00 000 |
    ///              | ---- --- --- - -- --- |
    ///              | op   dr  sr1 c -- sr2 |
    ///              +-----------------------+
    ///        AND - | 0101 000 000 1 00000  |
    ///              | ---- --- --- - -----  |
    ///              | op   dr  sr1 c imm5   |
    ///
    /// ## Legend:
    /// ```
    ///     op ---> opcode,
    ///     dr ---> desitination register
    ///     sr1 --> source register 1
    ///     sr2 --> source register 2
    ///     imm5 -> immediate value (5-bit 2's compliement)
    ///```
    ///
    /// # Example:
    ///
    ///     ADD R0, R1, R2 ; -> memory[R0] = memory[R1] & memory[R2]
    ///
    ///     ADD R0, R1, #2 ; -> memory[R0] = memory[R1] & 2
    ///
    /// **NOTE**:
    /// The immeidate value is only a 5-bit 2's complement number. Therefore the range accepted as
    /// an immediate value is [-16, 15].
    /// -------------------------------------------------------------------------------
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory, _io: &mut Lc3IO) {
        let mut i = value;

        let dr = i >> 9;
        i -= dr << 9;

        let sr1 = i >> 6;
        let x = sr1 << 6;
        i -= x;

        let code = get_bit_index(value, 5);

        let new_value: u16;

        match code {
            0 => {
                let sr2 = i;

                let v1 = reg.get(sr1 as usize);
                let v2 = reg.get(sr2 as usize);
                new_value = v1 & v2;
                // println!("\nand reg: {} & {} = {}\n", v1, v2, new_value);
            }
            1 => {
                // i -= code >> 5;
                let reg_val = reg.get(sr1 as usize);
                let imm_val = get_offset(value, 5);
                new_value = reg_val & imm_val;
                // println!("\nand imm: {} & {} = {}\n", reg_val, imm_val, new_value);
            }
            _ => {
                unreachable!();
            }
        }
        reg.set(dr as usize, new_value);

        set_nzp(reg, new_value);
    }
}

impl Instruction for Br {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        BR  - | 0000 000 000000000 |
              | ---- --- --------- |
              | op   nzp pcoffset9 |
        */
        // print!("IN BR");
        // print!(" - {:#018b}\n", value);
        let n = get_bit_index(value, 11);
        let z = get_bit_index(value, 10);
        let p = get_bit_index(value, 9);

        if (n == 1 && z == 1 && p == 1)
            || (n == 1 && reg.n)
            || (z == 1 && reg.z)
            || (p == 1 && reg.p)
        {
            let pcoffset9 = get_offset(value, 9);
            let target_location = get_pcoffset_location(&reg, pcoffset9);
            reg.pc = target_location;
        }
    }
}

impl Instruction for JmpRet {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        JMP - | 1100 000 000 000000 |
              | ---- --- --- ------ |
              | op       baser      |
              +---------------------+
        RET - | 1100 000 111 000000 |
              | ---- --- --- ------ |
              | op       r7         |
        */
        let base_reg = value >> 6;

        reg.pc = reg.get(base_reg as usize);
    }
}

impl Instruction for Jsr {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        JSR - | 0100 1 00000000000   |
              | ---- - -----------   |
              | op   c pcoffset11    |
              +----------------------+
        JSRR- | 0100 0 00 000 000000 |
              | ---- - -- --- ------ |
              | op   c -- br  ------ |
        */
        // let code = get_bit_index(value, 12);
        let code = value >> 11;
        let inc_pc = reg.pc;
        let offset: u16;

        match code {
            0 => {
                let offset_reg = value >> 6;
                offset = reg.r[offset_reg as usize];
            }
            1 => {
                offset = get_offset(value, 11);
            }
            _ => unreachable!(),
        }

        let target_location = get_pcoffset_location(&reg, offset);
        reg.pc = target_location;

        // link back to the instruction after Jsr by putting PC in R7
        reg.r[7] = inc_pc;
    }
}

impl Instruction for Ld {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        LD  - | 0010 000 000000000 |
              | ---- --- --------- |
              | op   dr  pcoffset9 |
        */
        let dr = value >> 9;
        let pcoffset9 = get_offset(value, 9);
        let relative_pc_address = get_pcoffset_location(reg, pcoffset9);

        let new_value = mem.get(relative_pc_address);
        set_nzp(reg, new_value);
        reg.set(dr as usize, new_value);
    }
}

impl Instruction for Ldi {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        LDI - | 1010 000 000000000 |
              | ---- --- --------- |
              | op   dr  pcoffset9 |
        */
        let dr = value >> 9;
        let pcoffset9 = get_offset(value, 9);
        let relative_pc_address = get_pcoffset_location(reg, pcoffset9); 

        let ptr = mem.get(relative_pc_address);
        let new_value = mem.get(ptr);
        set_nzp(reg, new_value);
        reg.set(dr as usize, new_value);
    }
}

impl Instruction for Ldr {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        LDR - | 0110 000 000 000000 |
              | ---- --- --- ------ |
              | op   dr  br  offset6|
        */
        let mut buffer = value;
        let dr = buffer >> 9;
        buffer -= dr << 9;
        let base_r = buffer >> 6;

        let offset = get_offset(value, 6);
        let address = reg.get(base_r as usize);

        let target_location = calculate_relative_offset(address, offset);

        let new_value = mem.get(target_location);
        set_nzp(reg, new_value);
        reg.set(dr as usize, new_value);
    }
}

/// Loads memory location of the label into memory
impl Instruction for Lea {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        LEA - | 1110 000 000000000 |
              | ---- --- --------- |
              | op   dr  label     |

        Loads memory location of the label into memory
        */
        let dr = value >> 9;
        let ptr = get_offset(value, 9);

        let address = get_pcoffset_location(reg, ptr);
        set_nzp(reg, address);
        reg.set(dr as usize, address);
    }
}

impl Instruction for Not {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        NOT - | 1001 000 000 111111 |
              | ---- --- --- ------ |
              | op   dr  sr         |
        */
        let mut i = value;
        let dr = i >> 9;
        i -= dr << 9;
        let sr = i >> 6;

        let old_val = reg.get(sr as usize);
        let not_val = !old_val;

        reg.set(dr as usize, not_val);

        set_nzp(reg, not_val);
    }
}

// TODO: Impl rti
impl Instruction for Rti {
    fn exe(&self, _value: u16, _reg: &mut Registers, _mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        RTI - | 1000 000000000000 |
              | ---- ------------ |
              | op                |
        */
        todo!();
    }
}

impl Instruction for St {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        ST  - | 0011 000 000000000 |
              | ---- --- --------- |
              | op   sr  pcoffset9 |
        */
        let sr = value >> 9;
        let pcoffset9 = get_offset(value, 9);
        let location = get_pcoffset_location(&reg, pcoffset9);

        mem.set(location, reg.get(sr as usize));
    }
}

impl Instruction for Sti {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        STI - | 1011 000 000000000 |
              | ---- --- --------- |
              | op   sr  pcoffset9 |
        */
        let sr = value >> 9;
        let pcoffset9 = get_offset(value, 9);
        let location = get_pcoffset_location(&reg, pcoffset9);
        let indirect = mem.get(location);

        mem.set(indirect, reg.get(sr as usize));
    }
}

impl Instruction for Str {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory, _io: &mut Lc3IO) {
        /*
        STR - | 0111 000 000 000000  |
              | ---- --- --- ------  |
              | op   sr      offset6 |
                         base_r
        */
        let mut buffer = value;
        let sr = buffer >> 9;
        buffer -= sr << 9;
        let base_r = buffer >> 6;
        let offset6 = get_offset(buffer, 6);

        let relative_offeset = calculate_relative_offset(reg.get(base_r as usize), offset6);

        mem.set(relative_offeset, reg.get(sr as usize));
    }
}

impl Instruction for Trap {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory, io: &mut Lc3IO) {
        /*
        TRAP - | 1111 0000 00000000 |
               | ---- ---- -------- |
               | op        trapvec8 |
        */
        let code = get_offset(value, 8);

        match code {
            GETC_VAL => self.get_c(reg, io),
            OUT_VAL => self.out(reg, io),
            PUTS_VAL => self.put_s(reg, mem, io),
            IN_VAL => self.r#in(reg, mem, io),
            HALT_VAL => self.halt(reg),
            _ => unreachable!(),
        }
    }
}

fn get_offset(mut value: u16, num_bits: i32) -> u16 {
    /*
    Every number passed here is a 2's complement signed integer.
    Therefore, we need to check if the right-most bit is a `1`.
    if true, exend entire number with ones.
    */
    let mut pos: u32 = 1;
    let mut buf: u16 = 0;
    let mut bit = 0;

    for _ in 0..num_bits {
        bit = (value % 2) * pos as u16;
        buf += bit;
        pos *= 2;
        value = value >> 1;
    }

    // value is negative if the last bit was not zero
    if bit > 1 {
        let remaining_bits = 16 - num_bits;

        for _ in 0..remaining_bits {
            buf += pos as u16;
            pos *= 2;
        }
    }

    return buf;
}

fn get_bit_index(value: u16, index: i32) -> u16 {
    return value >> index & 1;
}

fn set_nzp(reg: &mut Registers, value: u16) {
    reg.n = false;
    reg.z = false;
    reg.p = false;

    let signed = value as i16;

    if signed < 0 {
        reg.n = true;
    }
    if signed == 0 {
        reg.z = true;
    }
    if signed > 0 {
        reg.p = true;
    }
}

fn calculate_relative_offset(relative_location: u16, offset: u16) -> u16 {
    let value = relative_location as i16 + offset as i16;
    return value as u16;
}

fn get_pcoffset_location(reg: &Registers, value: u16) -> u16 {
    if value as i16 >= 0 {
        return reg.pc + value;
    }
    // if Rust ignored unused carried "overflow" bits, this wouldn't be necessary.
    // but it doesn't, so it is.
    let signed_value = value as i16;
    let negated_value = !signed_value + 1;

    return reg.pc - negated_value as u16;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let add = super::Add {};

        reg.set(0, 2);
        reg.set(1, 8);

        let ins: u16 = 0b0000_010_001_0_00_000;
        add.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(2) == 10);

        let ins: u16 = 0b0000_010_001_1_00011; // 3
        add.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(2) == 11);
        // TODO: Account for NZP bits

        assert!(reg.n == false);
        assert!(reg.z == false);
        assert!(reg.p == true);

        let ins: u16 = 0b0000_010_001_1_11000; // -8
        add.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(2) == 0);

        assert!(reg.n == false);
        assert!(reg.z == true);
        assert!(reg.p == false);

        let ins: u16 = 0b0000_010_000_1_11000; // R0 with -8
        add.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(2) as i16 == -6);

        assert!(reg.n == true);
        assert!(reg.z == false);
        assert!(reg.p == false);
    }

    #[test]
    fn test_and() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let and = super::And {};

        reg.set(0, 3);
        reg.set(1, 9);

        let mut ins: u16 = 0b0000_010_001_0_00_000;
        and.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(2) == 1);

        assert!(reg.n == false);
        assert!(reg.z == false);
        assert!(reg.p == true);

        ins = 0b0000_010_001_1_11001;
        and.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(2) == 9);

        ins = 0b0000_010_001_1_00000;
        reg.set(1, 13048);
        and.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.n == false);
        assert!(reg.z == true);
        assert!(reg.p == false);
        assert!(reg.get(2) == 0);

        let val1 = 0b0010_0110_1111_0101;
        let val2 = 0b1010_0111_1000_1010;
        let resv = 0b0010_0110_1000_0000;

        reg.set(0, val1);
        reg.set(1, val2);
        let ins = 0b0000_001_000_0_00_001;
        and.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == resv);
    }

    // #[test]
    // fn test_br() {
    //     unimplemented!();
    // }

    #[test]
    fn test_jmp() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let jmp = super::JmpRet {};

        let ins: u16 = 0b0000_000_001_000000;
        reg.pc = 16;
        reg.set(1, 4000);

        jmp.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.pc != 16);
        assert!(reg.pc == 4000);

        let ins: u16 = 0b0000_000_011_000000;
        reg.set(3, 2048);

        jmp.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.pc != 4000);
        assert!(reg.pc == 2048);
    }

    #[test]
    fn test_ret() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let jmp = super::JmpRet {};

        let ins: u16 = 0b0000_000_111_000000; // CAN NEVER CHANGE. RET is a completely static instruction
        reg.pc = 16;
        reg.set(7, 999);

        jmp.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.pc != 16);
        assert!(reg.pc == 999);

        reg.set(7, 2190);
        jmp.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.pc != 999);
        assert!(reg.pc == 2190);
    }

    // #[test]
    // fn test_jsr() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_jsrr() {
    //     unimplemented!();
    // }

    #[test]
    fn test_ld() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let ld = super::Ld {};
    
        reg.pc = 3000;
        let val: i16 = -16;
        let ins = 0b0000_001_111111111;
        mem.set(2999, val as u16);

        assert!(reg.get(1) != val as u16);
        assert!(reg.n != true);
        ld.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == val as u16);
        assert!(reg.n == true);

        let val: i16 = 32;
        mem.set(2999, val as u16);

        assert!(reg.get(1) != val as u16);
        assert!(reg.p != true);
        ld.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == val as u16);
        assert!(reg.p == true);
 

        let val: i16 = 0;
        mem.set(2999, val as u16);

        assert!(reg.get(1) != val as u16);
        assert!(reg.z != true);
        ld.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == val as u16);
        assert!(reg.z == true);
     }


    #[test]
    fn test_ldi() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let ldi = super::Ldi {};
    
        reg.pc = 3000;
        let ptr: u16 = 0x3145;
        mem.set(2999, ptr);

        let val: i16 = -16;
        let ins = 0b0000_001_111111111;
        mem.set(ptr, val as u16);

        assert!(reg.get(1) != val as u16);
        assert!(reg.n != true);
        ldi.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == val as u16);
        assert!(reg.n == true);

        let val: i16 = 32;
        mem.set(ptr, val as u16);

        assert!(reg.get(1) != val as u16);
        assert!(reg.p != true);
        ldi.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == val as u16);
        assert!(reg.p == true);
 

        let val: i16 = 0;
        mem.set(ptr, val as u16);

        assert!(reg.get(1) != val as u16);
        assert!(reg.z != true);
        ldi.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == val as u16);
        assert!(reg.z == true);
    }

    #[test]
    fn test_ldr() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let ldr = super::Ldr {};

        let location = 2999;
        let ins: u16 = 0b0000_001_010_111111;

        let val: i16 = -16;
        reg.set(2, 3000);
        mem.set(location, val as u16);

        assert!(reg.get(1) != val as u16);
        assert!(reg.n != true);
        ldr.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == val as u16);
        assert!(reg.n == true);

        let val: i16 = 32;
        mem.set(location, val as u16);

        assert!(reg.get(1) != val as u16);
        assert!(reg.p != true);
        ldr.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == val as u16);
        assert!(reg.p == true);
 

        let val: i16 = 0;
        mem.set(location, val as u16);

        assert!(reg.get(1) != val as u16);
        assert!(reg.z != true);
        ldr.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == val as u16);
        assert!(reg.z == true);
    }

    #[test]
    fn test_lea() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let lea = super::Lea {};

        reg.pc = 3000;
        let ins: u16 = 0b0000_001_111111111;

        assert!(reg.get(1) != 2999);
        assert!(reg.p != true);
        lea.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == 2999);
        assert!(reg.p == true);
 
        reg.pc = 1;

        assert!(reg.get(1) != 0);
        assert!(reg.z != true);
        lea.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(reg.get(1) == 0);
        assert!(reg.z == true);
    }

    #[test]
    fn test_not() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let not = super::Not {};

        reg.set(1, 0b0000_0101_0000_1111);

        let ins: u16 = 0b0000_000_001_111111;
        not.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(0) != reg.get(1));
        assert!(reg.get(0) == !reg.get(1));

        assert!(reg.n == true);
        assert!(reg.z == false);
        assert!(reg.p == false);

        reg.set(1, 0b0000_1111_0101_1010);

        let ins: u16 = 0b0000_000_001_111111;
        not.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(0) != reg.get(1));
        assert!(reg.get(0) == !reg.get(1));

        assert!(reg.n == true);
        assert!(reg.z == false);
        assert!(reg.p == false);

        reg.set(1, 0b1101_1011_1111_1110);
        not.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(0) != reg.get(1));
        assert!(reg.get(0) == !reg.get(1));

        assert!(reg.n == false);
        assert!(reg.z == false);
        assert!(reg.p == true);

        reg.set(1, 0b1111_1111_1111_1111);
        not.exe(ins, &mut reg, &mut mem, &mut io);

        assert!(reg.get(0) != reg.get(1));
        assert!(reg.get(0) == !reg.get(1));

        assert!(reg.n == false);
        assert!(reg.z == true);
        assert!(reg.p == false);
    }

    // #[test]
    // fn test_rti() {
    //     // unimplemented!(); going to implement in later version.
    // }

    #[test]
    fn test_st() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let st = super::St {};

        reg.pc = 3000;
        let ins: u16 = 0b0000_001_111111111;

        let location = 2999;
        let val: i16 = -16;
        reg.set(1, val as u16);

        assert!(mem.get(location) != val as u16);
        st.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(mem.get(location) == val as u16);


        let ins: u16 = 0b0000_001_000000001;
        let location = 3001;
        let val: i16 = 42;
        reg.set(1, val as u16);
        
        assert!(mem.get(location) != val as u16);
        st.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(mem.get(location) == val as u16);
    }

    #[test]
    fn test_sti() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let sti = super::Sti {};

        reg.pc = 3000;
        let ins: u16 = 0b0000_001_111111111;

        let location = 2999;
        let ptr: u16 = 0x12F2;
        let val: i16 = -16;
        mem.set(location, ptr);
        reg.set(1, val as u16);

        assert!(mem.get(ptr) != val as u16);
        sti.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(mem.get(ptr) == val as u16);


        let ins: u16 = 0b0000_001_000000001;
        let location = 3001;
        let ptr = 0x0A1C;
        let val: i16 = 42;
        mem.set(location, ptr);
        reg.set(1, val as u16);
        
        assert!(mem.get(ptr) != val as u16);
        sti.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(mem.get(ptr) == val as u16);
    }

    #[test]
    fn test_str() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let str = super::Str {};

        reg.pc = 3000;
        let ins: u16 = 0b0000_001_111_111111;

        let ptr: u16 = 0x12F2;
        let val: i16 = -16;
        reg.set(1, val as u16);
        reg.set(7, ptr);

        assert!(mem.get(ptr - 1) != val as u16);
        str.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(mem.get(ptr - 1) == val as u16);


        let ins: u16 = 0b0000_001_111_000001;
        let ptr: u16 = 0x0A1C;
        let val: u16 = 42;
        reg.set(1, val);
        reg.set(7, ptr);
        
        assert!(mem.get(ptr + 1) != val as u16);
        str.exe(ins, &mut reg, &mut mem, &mut io);
        assert!(mem.get(ptr + 1) == val as u16);
    }

    #[test]
    fn test_set_nzp() {
        let mut reg = Registers::new();

        set_nzp(&mut reg, u16::MAX);
        assert!(reg.n);
        assert!(!reg.z);
        assert!(!reg.p);

        set_nzp(&mut reg, 0);
        assert!(!reg.n);
        assert!(reg.z);
        assert!(!reg.p);

        set_nzp(&mut reg, 1);
        assert!(!reg.n);
        assert!(!reg.z);
        assert!(reg.p);
    }

    #[test]
    fn test_negative_pc_offsets() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        reg.pc = 3000;

        let br = Br {};
        br.exe(0b0000_111_111111111, &mut reg, &mut mem, &mut io);

        let jsr = Jsr {};
        jsr.exe(0b0000_1_11111111111, &mut reg, &mut mem, &mut io);
        jsr.exe(0b0000_0_00_111111111, &mut reg, &mut mem, &mut io);

        let ld = Ld {};
        ld.exe(0b0000_111_111111111, &mut reg, &mut mem, &mut io);

        let ldi = Ldi {};
        ldi.exe(0b0000_111_111111111, &mut reg, &mut mem, &mut io);

        mem.set(u16::MAX, u16::MAX);
        reg.set(7, u16::MAX);
        let ldr = Ldr {};
        ldr.exe(0b0000_111_111_111111, &mut reg, &mut mem, &mut io);
    }

    #[test]
    fn test_load_ins_set_nzp() {

    }
}
