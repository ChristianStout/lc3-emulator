use super::asm_ins::*;
use super::directive::*;
use super::syntax::SyntaxChecker;
use super::asm_error::*;
use super::token::*;

pub struct Lexer {
    pub token_stream: Vec<Token>,
    pub errors: Vec<AsmError>,
    syntax_checker: SyntaxChecker,
    curr_line_num: i32,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            token_stream: vec![],
            errors: vec![],
            syntax_checker: SyntaxChecker::new(),
            curr_line_num: 0,
        }
    }

    pub fn run(&mut self, input_file: Vec<&str>) -> Vec<Token> {
        for line in input_file {
            self.curr_line_num += 1;
            // println!("LINE {}: {line}", self.curr_line_num);

            if self.syntax_checker.is_ins(line) || self.syntax_checker.is_dir(line) {
                self.parse_line(line);
                continue;
            }
            if !self.syntax_checker.is_ignore(line) {
                self.errors.push(AsmError::new(
                    line,
                    self.curr_line_num,
                    ErrorType::SyntaxError,
                    "neither a valid instruction nor a valid directive line was given. perhaps there are too many operands?"
                ))
            }
        }

        // TODO: Rid yourself of this HORRID thing D8< (the clone) (just don't return it and get it from the outside?)
        return self.token_stream.clone();
    }

    pub fn parse_line(&mut self, line: &str) {
        let split_line: Vec<&str> = line.split_whitespace().collect();

        for word in split_line {

            // parse hierarchy

            if self.syntax_checker.is_ignore(word) {
                // println!("Ignored word: {word}");
                return;
            }
            if self.syntax_checker.is_instruction_name(word) {
                self.token_stream.push(Token::Instruction(OpcodeIns::from(word)));
                continue;
            }
            if self.syntax_checker.is_directive_name(word) {
                self.token_stream.push(Token::Directive(Directive::from(word)));
                continue;
            }
            if self.syntax_checker.is_valid_register(word) {
                self.token_stream.push(Token::Register(self.parse_register(word)));
                continue;
            }
            if self.syntax_checker.is_valid_immediate_value(word) {
                self.token_stream.push(Token::Number(self.parse_immediate_value(word)));
                continue;
            }
            if self.syntax_checker.is_valid_label(word) {
                self.token_stream.push(Token::Label(word.to_string()));
                continue;
            }

            // TODO: Parse Strings. this will require the ability to extract the
            // string from the original line

            // if self.syntax_checker.is_valid_string(word) {
            //     self.token_stream.push(Token::Number(self.parse_string(word)));
            //     continue;
            // }
        }
    }

    pub fn parse_register(&self, word: &str) -> u16 {
        let base = 10;

        let register_num = word
            .chars()
            .nth(1) // Always R1, R2, R3, ... , R7.
            .expect("Lexer::parse_register: Somehow a register was given without a number. This shouldn't be possible given the Regex.")
            .to_digit(base) // a base 10 number
            .expect(&format!("Lexer::parse_register: When converting the register value on line {}, could not conver value into base 10 number.", self.curr_line_num));
        
        return register_num as u16;
    }
    
    pub fn parse_immediate_value(&self, word: &str) -> i16 {
        match word.chars().nth(0).unwrap() {
            '#' => {
                return word[1..]
                    .parse()
                    .expect(&format!("Lexer::parse_immediate_value: The given number on line {} is not valid", self.curr_line_num));
            },
            'x' | 'X' => {
                let base = 16;
                return u16::from_str_radix(&word[1..], base)
                    .expect(&format!("Lexer::parse_immediate_value: The given number on line {} is not valid", self.curr_line_num)) as i16;
            },
            _ => unreachable!(),
        }
    }

    // pub fn parse_string(&self, line: &str) -> String {
    //     let mut i = 0;
    //     let mut in_string = false;
    //     let mut str_buffer = "";

    //     for c in line.chars() {
    //         if !in_string {
    //             match c {
    //                 '\"' => {
    //                     in_string = true;
    //                     continue;
    //                 },
    //                 _ => continue,
    //             }
    //         }



    //         i += 1;
    //     }

    //     return str_buffer.to_string();
    // }

    // pub fn parse_escape(&self, character: char) -> char {

    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_overflow() {
        let file = String::from("NOT_TOO_BIG  .FILL   xFFF6\nEVEN_THIS .FILL xFFFF");

        let mut lexer = Lexer::new();
        let tokens = lexer.run(file.split_ascii_whitespace().collect());

        assert!(tokens[2] == Token::Number(65526_u16 as i16));
        assert!(tokens[5] == Token::Number(65535_u16 as i16));
    }
}
