use super::{memory::Memory, registers::Registers};
use crate::io::Lc3IO;
pub struct Trap;

impl Trap {
    /// takes a single char as input from the console and puts it in R0
    pub fn get_c(&self, reg: &mut Registers, io: &mut Lc3IO) {
        let c = io.get_char();

        reg.set(0, c as u16);
    }

    /// Outputs the value in R0 as a char to the console
    pub fn out(&self, reg: &mut Registers, io: &mut Lc3IO) {
        io.print_single_char(reg);
    }

    /// prints a string to the console pointed to by R0
    pub fn put_s(&self, reg: &mut Registers, mem: &mut Memory, io: &mut Lc3IO) {
        io.print_string(reg, mem);
    }

    /// Prints a prompt string pointed to by R0,
    /// then takes a single char as input from the console and puts it in R0
    pub fn r#in(&self, reg: &mut Registers, mem: &mut Memory, io: &mut Lc3IO) {
        io.print_string(reg, mem);

        let c = io.get_char();
        reg.set(0, c as u16);
    }

    /// Triggers the halt register to signal to the VM to end the program
    pub fn halt(&self, reg: &mut Registers) {
        println!("\nHALT TRIGGERED!");
        reg.halt = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::registers::Registers;

    #[test]
    fn test_out() {
        use crate::io;
        let mut io = super::Lc3IO::new(Box::new(io::StdIOTarget {}));
        let mut reg = Registers::new();
        let trap = Trap {};

        reg.set(0, 'a' as u16);

        trap.out(&mut reg, &mut io);

        reg.set(0, 'p' as u16);

        trap.out(&mut reg, &mut io);
        trap.out(&mut reg, &mut io);
    }
}
