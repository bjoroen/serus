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
                '#' => self.parse_operand(),
                '$' => self.parse_register(),
                _ if c.is_alphabetic() => self.parse_opcode(c),
                _ if c.is_whitespace() => {}
                _ => unimplemented!(),
            }
        }
    }

    fn parse_opcode(&mut self, c: char) {
        let mut s = String::from(c);
        let line = self.current_line;

        while let Some(c) = self.next() {
            if c.is_alphabetic() {
                s.push(c)
            } else {
                break;
            }
        }

        if let Ok(opcode) = Opcode::from_str(&s.to_lowercase()) {
            let token = Token::Op { code: opcode, line };
            self.tokens.push(token)
        } else {
            eprintln!("Unknown Opcode: {} on line {}", s, self.current_line)
        }
    }

    fn parse_register(&mut self) {
        let mut s = vec![];
        let line = self.current_line;

        while let Some(c) = self.next() {
            if c.is_alphanumeric() {
                s.push(c)
            } else {
                break;
            }
        }

        let register = String::from_iter(s);

        self.tokens.push(Token::Register { register, line })
    }

    fn parse_operand(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn run_test(test_cases: &[(&str, Vec<&str>)]) {
        for case in test_cases {
            let mut lexer = Lexer::new(case.0);
            lexer.lex();

            for (i, token) in lexer.tokens.into_iter().enumerate() {
                assert_eq!(token.to_string(), case.1[i])
            }
        }
    }

    #[test]
    fn test_lex_opcode() {
        let test_cases = [("LOAD\nADD]\nSUB", vec!["1:load", "2:add", "3:sub"])];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_register() {
        let test_cases = [("$10\n$hello\n$world", vec!["10", "hello", "world"])];

        run_test(&test_cases)
    }
}
