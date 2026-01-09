use super::{memory::Memory, registers::Registers};
use crate::io::Lc3IO;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use tsify::Tsify;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize, Tsify))]
#[cfg_attr(feature = "serde", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Trap;

impl Trap {
    /// takes a single char as input from the console and puts it in R0
    pub fn get_c(&self, reg: &mut Registers, io: &mut Lc3IO) {
        let c = io.get_char();

        reg.set(0, c as u16);
    }

    /// Outputs the value in R0 as a char to the console
    pub fn out(&self, reg: &mut Registers, io: &mut Lc3IO) {
        let c = reg.get(0) as u8 as char;
        io.put_char(c);
    }

    /// prints a string to the console pointed to by R0
    pub fn put_s(&self, reg: &mut Registers, mem: &mut Memory, io: &mut Lc3IO) {
        let mut i = reg.get(0);
        let mut c = mem.get(i) as u8 as char;

        while c != '\0' {
            io.put_char(c);
            i += 1;
            c = mem.get(i) as u8 as char;
        }
    }

    pub fn put_sp(&self, reg: &mut Registers, mem: &mut Memory, io: &mut Lc3IO) {
        // io.print_string_special(reg, mem);
        unimplemented!();
    }

    /// Prints a prompt string pointed to by R0,
    /// then takes a single char as input from the console and puts it in R0
    pub fn r#in(&self, reg: &mut Registers, mem: &mut Memory, io: &mut Lc3IO) {
        let mut i = reg.get(0);
        let mut c = mem.get(i) as u8 as char;

        while c != '\0' {
            io.put_char(c);
            i += 1;
            c = mem.get(i) as u8 as char;
        }

        let c = io.get_char();
        reg.set(0, c as u16);
    }

    /// Triggers the halt register to signal to the VM to end the program
    pub fn halt(&self, reg: &mut Registers) {
        // println!("\nHALT TRIGGERED!");
        reg.halt = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::registers::Registers;
    use crate::io::*;

    #[test]
    fn test_out() {
        let mut io = Lc3IO::new(Box::new(DebugIO::new()));
        let mut reg = Registers::new();
        let trap = Trap {};

        reg.set(0, 'a' as u16);

        trap.out(&mut reg, &mut io);

        reg.set(0, 'p' as u16);

        trap.out(&mut reg, &mut io);
        trap.out(&mut reg, &mut io);
    }
}
