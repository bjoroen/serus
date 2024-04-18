#![allow(dead_code)]
use std::str::{Chars, FromStr};

use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Op { code: Opcode },
    Register { register: usize },
    IntOperand { operand: usize },
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    line: usize,
}

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
        match self.source.next() {
            Some(c) => {
                if c == '\n' {
                    self.current_line += 1
                }
                Some(c)
            }
            None => None,
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

        while let Some(c) = self.next() {
            if c.is_alphabetic() {
                s.push(c)
            } else {
                break;
            }
        }

        if let Ok(opcode) = Opcode::from_str(&s.to_lowercase()) {
            let token = Token {
                token_type: TokenType::Op { code: opcode },
                line: self.current_line,
            };
            self.tokens.push(token)
        } else {
            eprintln!("Unknown Opcode: {} on line {}", s, self.current_line)
        }
    }

    fn parse_register(&self) {
        todo!()
    }

    fn parse_operand(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode() {
        let mut lexer = Lexer::new("LOAD");
        lexer.lex();
        assert_eq!(
            lexer.tokens[0],
            Token {
                token_type: TokenType::Op { code: Opcode::LOAD },
                line: 1
            }
        )
    }
}
