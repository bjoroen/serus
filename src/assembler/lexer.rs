#![allow(dead_code)]
use std::str::FromStr;

use crate::instruction::Opcode;

use super::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    source: Vec<char>,
    current: usize,
    next: usize,
    char: char,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        let source = source.chars().collect();
        let mut l = Lexer {
            source,
            current: 0,
            next: 1,
            char: '\0',
        };

        l.char = l.source[l.current];
        l
    }

    fn read(&mut self) {
        if self.next >= self.source.len() {
            self.char = '\0';
        } else {
            self.char = self.source[self.next];
        }

        self.current = self.next;
        self.next = self.current + 1;
    }

    pub fn lex(&mut self) -> Token {
        while self.char.is_whitespace() {
            self.read()
        }

        match self.char {
            '#' => self.parse_int_operand(),
            '$' => self.parse_register(),
            c if c.is_alphabetic() => self.parse_opcode(),
            '\0' => Token::EOF,
            _ => unimplemented!(),
        }
    }

    fn parse_opcode(&mut self) -> Token {
        let mut s = String::new();

        while self.char.is_alphabetic() {
            s.push(self.char);
            self.read();
        }

        if let Ok(opcode) = Opcode::from_str(&s.to_lowercase()) {
            let token = Token::Op { code: opcode };

            token
        } else {
            panic!("Unknown Opcode: {}", s)
        }
    }

    fn parse_register(&mut self) -> Token {
        let mut s = String::new();
        self.read();

        while self.char.is_numeric() {
            s.push(self.char);
            self.read()
        }

        if let Ok(register) = s.parse() {
            Token::Register { register }
        } else {
            panic!("Could not parse number for register: {}", s)
        }
    }

    fn parse_int_operand(&mut self) -> Token {
        let mut s = String::new();
        self.read();

        while self.char.is_numeric() {
            s.push(self.char);
            self.read()
        }

        if let Ok(int_operand) = s.parse() {
            Token::IntOperand {
                operand: int_operand,
            }
        } else {
            panic!("Could not parse number for register: {}", s)
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.current >= self.source.len() {
            return None;
        }

        let token = self.lex();

        Some(token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn run_test(test_cases: &[(&str, &str)]) {
        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let mut tokens = vec![];

            while let Some(t) = lexer.next() {
                tokens.push(t)
            }

            let token_string = String::from_iter(tokens.iter().map(|t| t.to_string()));

            assert_eq!(token_string, *expected)
        }
    }

    #[test]
    fn test_lex_opcode() {
        let test_cases = [("LOAD", "load "), ("\nADD", "add "), ("\n\nSUB", "sub ")];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_register() {
        let test_cases = [("$10", "10 "), ("$100", "100 "), ("$5", "5 ")];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_operand() {
        let test_cases = [("#10", "10 "), ("#20", "20 "), ("#30", "30 ")];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_instruction() {
        let test_cases = [
            ("load $0 #10", "load 0 10 "),
            ("add $0 $2 #1", "add 0 2 1 "),
            ("load $1 #10\n", "load 1 10 "),
            ("sub $0 #1 #2\n add $2 #30 #20", "sub 0 1 2 add 2 30 20 "),
        ];

        run_test(&test_cases)
    }
}
