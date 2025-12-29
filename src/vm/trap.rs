use super::{memory::Memory, registers::Registers};
use crossterm::terminal;
use std::io::*;
pub struct Trap;

impl Trap {
    /// takes a single char as input from the console and puts it in R0
    pub fn get_c(&self, reg: &mut Registers) {
        let c = self.get_char();
        reg.set(0, c as u16);
    }

    /// Outputs the value in R0 as a char to the console
    pub fn out(&self, reg: &mut Registers) {
        print!("{}", reg.get(0) as u8 as char);
    }

    /// prints a string to the console pointed to by R0
    pub fn put_s(&self, reg: &mut Registers, mem: &mut Memory) {
        self.print_string(reg, mem);
    }

    /// Prints a prompt string pointed to by R0,
    /// then takes a single char as input from the console and puts it in R0
    pub fn r#in(&self, reg: &mut Registers, mem: &mut Memory) {
        self.print_string(reg, mem);

        let c = self.get_char();
        reg.set(0, c as u16);
    }

    /// Triggers the halt register to signal to the VM to end the program
    pub fn halt(&self, reg: &mut Registers) {
        println!("\nHALT TRIGGERED!");
        reg.halt = true;
    }

    fn print_string(&self, reg: &mut Registers, mem: &mut Memory) {
        let mut i = reg.get(0);
        let mut c = mem.get(i) as u8 as char;

        while c != '\0' {
            print!("{c}");
            i += 1;
            c = mem.get(i) as u8 as char;
        }
    }

    fn get_char(&self) -> char {
        terminal::enable_raw_mode().ok();
        let input = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as i64)
            .expect("Expected to reciece a value from the terminal, and got nothing");
        terminal::disable_raw_mode().ok();

        let c = input as u8 as char;
        print!("{c}");
        return c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::registers::Registers;

    #[test]
    fn test_out() {
        let mut reg = Registers::new();
        let trap = Trap {};

        reg.set(0, 'a' as u16);

        trap.out(&mut reg);

        reg.set(0, 'p' as u16);

        trap.out(&mut reg);
        trap.out(&mut reg);
    }
}
