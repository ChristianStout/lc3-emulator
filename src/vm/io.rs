use super::memory::Memory;
use super::registers::Registers;
use crossterm::event::*;
use crossterm::terminal;
use std::io::*;

#[allow(unused)]
pub struct Lc3IO {
    stdin: Vec<u8>,
    stdout: Vec<u8>,
    target: Box<dyn IOTarget>,
}

pub trait IOTarget {
    fn get_char(&self) -> char;
    fn print_string(&self, reg: &mut Registers, mem: &mut Memory);
    fn print_single_char(&self, reg: &mut Registers);
    fn print_error(&self, error_name: &str, error_msg: &str);
}

pub struct StdIOTarget;

impl Lc3IO {
    pub fn new(target: Box<dyn IOTarget>) -> Lc3IO {
        Lc3IO {
            stdin: Vec::new(),
            stdout: Vec::new(),
            target: target,
        }
    }

    pub fn get_char(&self) -> char {
        return self.target.get_char();
    }

    pub fn print_string(&self, reg: &mut Registers, mem: &mut Memory) {
        self.target.print_string(reg, mem);
    }

    pub fn print_single_char(&self, reg: &mut Registers) {
        self.target.print_single_char(reg);
    }

    pub fn print_error(&self,  error_name: &str, error_msg: &str) {
        self.target.print_error(error_name, error_msg);
    }
}

impl IOTarget for StdIOTarget {
    fn get_char(&self) -> char {
        terminal::enable_raw_mode().expect("Expected to be able to enter raw mode in get_char()");

        let out_c: char;

        // yes, it is this complicated actually.
        // the LC-3 requires that a char is read from
        // stdin but **not** displayed to the terminal
        // and **without** pressing enter.
        // This requires us to go into raw mode.
        // Because of this, we cannot quit out while waiting
        // for input, so we have to handle ctrl+c manually.
        loop {
            if let Ok(read_event) = read() {
                match read_event {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        match key.code {
                            // Ctrl+C should exit normally
                            KeyCode::Char('c')
                                if key
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
                            {
                                terminal::disable_raw_mode().expect(
                                    "Expected to be able to disable raw mode after pressing ctrl+c",
                                );
                                std::process::exit(130); // standard Ctrl+C exit code
                            }

                            // Normal character
                            KeyCode::Char(c) => {
                                out_c = c;
                                break;
                            }

                            // Optional: handle Enter explicitly if you want
                            KeyCode::Enter => {
                                out_c = '\n';
                                break;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        terminal::disable_raw_mode()
            .expect("Expected to be able to turn off raw mode after getting a char from stdin");
        return out_c;
        // // terminal::enable_raw_mode().ok();
        // let input = std::io::stdin()
        //     .bytes()
        //     .next()
        //     .and_then(|result| result.ok())
        //     .map(|byte| byte as i64)
        //     .expect("Expected to reciece a value from the terminal, and got nothing");
        // // terminal::disable_raw_mode().ok();

        // let c = input as u8 as char;
        // // print!("{c}");
        // return c;
    }

    fn print_string(&self, reg: &mut Registers, mem: &mut Memory) {
        let mut i = reg.get(0);
        let mut c = mem.get(i) as u8 as char;

        while c != '\0' {
            print!("{c}");
            i += 1;
            c = mem.get(i) as u8 as char;
        }
        std::io::stdout()
            .flush()
            .expect("Expected to be able to flush stdout after printing a string to the console.");
    }

    fn print_single_char(&self, reg: &mut Registers) {
        print!("{}", reg.get(0) as u8 as char);
        std::io::stdout().flush().expect(
            "Expected to be able to flush stdout after printing a char to the console in out().",
        );
    }

    fn print_error(&self,  error_name: &str, error_msg: &str) {
        println!("{error_name}: {error_msg}\n");
    }
}
