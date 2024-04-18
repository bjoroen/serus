#![allow(dead_code)]
use std::str::{Chars, FromStr};

use crate::instruction::Opcode;

use super::Token;

struct Lexer<'a> {
    source: Chars<'a>,
    pos: usize,
    pub tokens: Vec<Token>,
    current_line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer {
        let source = source.chars();
        Lexer {
            source,
            pos: 0,
            tokens: vec![],
            current_line: 1,
        }
    }

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.source.next() {
            if c == '\n' {
                self.current_line += 1
            }
            Some(c)
        } else {
            None
        }
    }

    pub fn lex(&mut self) {
        while let Some(c) = self.next() {
            match c {
                '#' => self.parse_int_operand(),
                '$' => self.parse_register(),
                _ if c.is_alphabetic() => self.parse_opcode(c),
                _ if c.is_whitespace() => {}
                _ => unimplemented!(),
            }
        }
    }

    fn parse_opcode(&mut self, c: char) {
        let mut s = String::from(c);

        while let Some(c) = self.next() {
            if c.is_alphabetic() {
                s.push(c)
            } else {
                assert!(c.is_whitespace());
                break;
            }
        }

        if let Ok(opcode) = Opcode::from_str(&s.to_lowercase()) {
            let token = Token::Op {
                code: opcode,
                line: self.current_line,
            };
            self.tokens.push(token)
        } else {
            panic!("Unknown Opcode: {} on line {}", s, self.current_line)
        }
    }

    fn parse_register(&mut self) {
        let mut s = vec![];

        while let Some(c) = self.next() {
            if c.is_alphanumeric() {
                s.push(c)
            } else {
                assert!(c.is_whitespace());
                break;
            }
        }

        let register = String::from_iter(s);

        self.tokens.push(Token::Register { register })
    }

    fn parse_int_operand(&mut self) {
        let mut s = String::new();

        while let Some(c) = self.next() {
            dbg!(&c);
            if c.is_numeric() {
                s.push(c)
            } else {
                break;
            }
        }

        self.tokens.push(Token::IntOperand {
            operand: s.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn run_test(test_cases: &[(&str, &str)]) {
        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            lexer.lex();

            let token_string = String::from_iter(lexer.tokens.iter().map(|t| t.to_string()));

            assert_eq!(token_string, *expected)
        }
    }

    #[test]
    fn test_lex_opcode() {
        let test_cases = [
            ("LOAD", "1:load "),
            ("\nADD", "2:add "),
            ("\n\nSUB", "3:sub "),
        ];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_register() {
        let test_cases = [("$10", "10 "), ("$hello", "hello "), ("$world", "world ")];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_operand() {
        let test_cases = [("#10", "10 "), ("#20", "20 "), ("#30", "30 ")];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_full_line() {
        let test_cases = [
            ("load $hello #10", "1:load hello 10 "),
            ("add $result $hello #1", "1:add result hello 1 "),
            (
                "sub $result #1 #2\n add $result #30 #20",
                "1:sub result 1 2 2:add result 30 20 ",
            ),
        ];

        run_test(&test_cases)
    }
}
