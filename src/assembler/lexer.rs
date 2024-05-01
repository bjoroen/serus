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
            '#' => self.lex_int_operand(),
            '$' => self.lex_register(),
            '@' => self.lex_label(),
            '.' => self.lex_directives(),
            _ if self.char.is_alphabetic() => self.parse_opcode(),
            '\0' => Token::EOF,
            // TODO: Better error handling in lexer
            _ => unimplemented!(),
        }
    }

    fn parse_opcode(&mut self) -> Token {
        let mut s = String::new();

        while self.char.is_alphabetic() || self.char == '_' || self.char == '-' {
            s.push(self.char);
            self.read();
        }

        if self.char == ':' {
            self.read();
            let token = Token::LabelDeclaration { value: s };

            token
        } else {
            if let Ok(opcode) = Opcode::from_str(&s.to_lowercase()) {
                let token = Token::Op { code: opcode };

                token
            } else {
                panic!("Unknown Opcode: {}", s)
            }
        }
    }

    fn lex_register(&mut self) -> Token {
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

    fn lex_int_operand(&mut self) -> Token {
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

    fn lex_label(&mut self) -> Token {
        self.read();
        let mut s = String::new();

        while self.char.is_alphabetic() || self.char == '_' || self.char == '-' {
            s.push(self.char);
            self.read()
        }

        Token::Label { name: s }
    }

    fn lex_directives(&mut self) -> Token {
        self.read();
        let mut s = String::new();

        while self.char.is_alphabetic() {
            s.push(self.char);
            self.read()
        }

        Token::Directive { value: s }
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

    fn run_test(test_cases: &[(&str, Token)]) {
        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let mut tokens = vec![];

            while let Some(t) = lexer.next() {
                tokens.push(t)
            }

            for token in tokens {
                assert_eq!(token, *expected)
            }
        }
    }

    #[test]
    fn test_lex_opcode() {
        let test_cases = [
            ("LOAD", Token::Op { code: Opcode::LOAD }),
            ("\nADD", Token::Op { code: Opcode::ADD }),
            ("\n\nSUB", Token::Op { code: Opcode::SUB }),
        ];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_register() {
        let test_cases = [
            ("$10", Token::Register { register: 10 }),
            ("$100", Token::Register { register: 100 }),
            ("$5", Token::Register { register: 5 }),
        ];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_operand() {
        let test_cases = [
            ("#10", Token::IntOperand { operand: 10 }),
            ("#20", Token::IntOperand { operand: 20 }),
            ("#30", Token::IntOperand { operand: 30 }),
        ];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_label_declaration() {
        let test_cases = [
            (
                "my_str:",
                Token::LabelDeclaration {
                    value: String::from("my_str"),
                },
            ),
            (
                "str:",
                Token::LabelDeclaration {
                    value: String::from("str"),
                },
            ),
            (
                "word:",
                Token::LabelDeclaration {
                    value: String::from("word"),
                },
            ),
        ];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_label() {
        let test_cases = [
            (
                "@my_str",
                Token::Label {
                    name: String::from("my_str"),
                },
            ),
            (
                "@str",
                Token::Label {
                    name: String::from("str"),
                },
            ),
            (
                "@word",
                Token::Label {
                    name: String::from("word"),
                },
            ),
        ];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_directives() {
        let test_cases = [
            (
                ".asciiz",
                Token::Directive {
                    value: String::from("asciiz"),
                },
            ),
            (
                ".data",
                Token::Directive {
                    value: String::from("data"),
                },
            ),
            (
                ".word",
                Token::Directive {
                    value: String::from("word"),
                },
            ),
        ];

        run_test(&test_cases)
    }

    #[test]
    fn test_lex_instruction() {
        let test_cases = [
            (
                "sub $0 #1 #2\n add $2 #30 #20",
                vec![
                    Token::Op { code: Opcode::SUB },
                    Token::Register { register: 0 },
                    Token::IntOperand { operand: 1 },
                    Token::IntOperand { operand: 2 },
                    Token::Op { code: Opcode::ADD },
                    Token::Register { register: 2 },
                    Token::IntOperand { operand: 30 },
                    Token::IntOperand { operand: 20 },
                ],
            ),
            (
                "load $0 #10",
                vec![
                    Token::Op { code: Opcode::LOAD },
                    Token::Register { register: 0 },
                    Token::IntOperand { operand: 10 },
                ],
            ),
            (
                "add $1 #20",
                vec![
                    Token::Op { code: Opcode::ADD },
                    Token::Register { register: 1 },
                    Token::IntOperand { operand: 20 },
                ],
            ),
            (
                "load $10 #20",
                vec![
                    Token::Op { code: Opcode::LOAD },
                    Token::Register { register: 10 },
                    Token::IntOperand { operand: 20 },
                ],
            ),
        ];

        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let mut tokens = vec![];

            while let Some(t) = lexer.next() {
                tokens.push(t)
            }

            for (i, token) in tokens.iter().enumerate() {
                assert_eq!(token, &expected[i])
            }
        }
    }
}
