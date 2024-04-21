use std::fmt::Display;

use crate::instruction::Opcode;

mod assembler_instruction;
mod lexer;
mod parser;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Op { code: Opcode },
    Register { register: i32 },
    IntOperand { operand: i32 },
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Op { code } => write!(f, "{} ", code),
            Token::Register { register } => write!(f, "{} ", register),
            Token::IntOperand { operand } => write!(f, "{} ", operand),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

impl From<Token> for u8 {
    fn from(t: Token) -> u8 {
        match t {
            Token::Op { code } => code as u8,
            Token::Register { register } => register as u8,
            Token::IntOperand { operand } => operand as u8,
            Token::EOF => todo!("Handle Error"),
        }
    }
}
