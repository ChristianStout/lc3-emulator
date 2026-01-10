use crossterm::event::*;
use crossterm::terminal;
use lc3::io::IOTarget;
use std::io::*;

pub struct StdIOTarget;

impl IOTarget for StdIOTarget {
    fn get_char(&mut self) -> char {
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
    }

    fn put_char(&mut self,c:char) {
        print!("{}", c);
        std::io::stdout().flush().expect(
            "Expected to be able to flush stdout after printing a char to the console in out().",
        );
    }
}
