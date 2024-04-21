#![allow(dead_code)]

use super::{assembler_instruction::AssemblerInstruction, lexer::Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    pub program: Vec<AssemblerInstruction>,
}

impl Parser {
    pub fn new(source_code: &str) -> Parser {
        let lexer = Lexer::new(source_code);
        Parser {
            lexer,
            current: Token::EOF,
            program: vec![],
        }
    }

    pub fn parse(&mut self) {
        self.read();

        while let Some(instruction) = self.next() {
            self.program.push(instruction);
        }
    }

    fn parse_instruction(&mut self) -> Option<AssemblerInstruction> {
        match self.current {
            Token::Op { code: _ } => {
                let op = self.current;

                // Eat the OP token
                self.read();

                Some(AssemblerInstruction {
                    opcode: op,
                    operand_one: self.next_operand(),
                    operand_two: self.next_operand(),
                    operand_three: self.next_operand(),
                })
            }
            Token::EOF => None,
            _ => panic!("Unexpected Token, instruction has to start with Opcode"),
        }
    }

    fn next_operand(&mut self) -> Option<Token> {
        match self.current {
            Token::Register { register: _ } => {
                let current = self.current;
                self.read();
                return Some(current);
            }
            Token::IntOperand { operand: _ } => {
                let current = self.current;
                self.read();
                return Some(current);
            }
            _ => None,
        }
    }

    fn next(&mut self) -> Option<AssemblerInstruction> {
        if self.current == Token::EOF {
            return None;
        }

        self.parse_instruction()
    }

    /// Reads and eats the next token
    pub fn read(&mut self) {
        self.current = if let Some(token) = self.lexer.next() {
            token
        } else {
            Token::EOF
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_instruction() {
        let mut p = Parser::new("load $10 #10\nHLT\nADD $0 $10 $5");
        p.parse();

        assert_eq!(p.program.len(), 3);

        assert_eq!(
            p.program[0],
            AssemblerInstruction {
                opcode: Token::Op {
                    code: crate::instruction::Opcode::LOAD
                },
                operand_one: Some(Token::Register { register: 10 }),
                operand_two: Some(Token::IntOperand { operand: 10 }),
                operand_three: None
            }
        );

        assert_eq!(
            p.program[1],
            AssemblerInstruction {
                opcode: Token::Op {
                    code: crate::instruction::Opcode::HLT
                },
                operand_one: None,
                operand_two: None,
                operand_three: None
            }
        );

        assert_eq!(
            p.program[2],
            AssemblerInstruction {
                opcode: Token::Op {
                    code: crate::instruction::Opcode::ADD
                },
                operand_one: Some(Token::Register { register: 0 }),
                operand_two: Some(Token::Register { register: 10 }),
                operand_three: Some(Token::Register { register: 5 }),
            }
        )
    }
}
