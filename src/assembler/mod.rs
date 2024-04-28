use std::fmt::Display;

use crate::instruction::Opcode;

mod assembler_instruction;
mod lexer;
mod parser;
pub mod program;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op { code: Opcode },
    Register { register: i32 },
    IntOperand { operand: i32 },
    Label { name: String },
    Directive { name: String },
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Op { code } => write!(f, "{} ", code),
            Token::Register { register } => write!(f, "{} ", register),
            Token::IntOperand { operand } => write!(f, "{} ", operand),
            Token::Label { name } => write!(f, "{} ", name),
            Token::Directive { name } => write!(f, ".{}", name),
            Token::EOF => write!(f, ""),
        }
    }
}

impl From<Token> for u8 {
    fn from(t: Token) -> u8 {
        match t {
            Token::Op { code } => code as u8,
            Token::Register { register } => register as u8,
            Token::IntOperand { operand } => operand as u8,
            Token::Label { name } => name as u8,
            Token::Directive { name } => name as u8,
            Token::EOF => todo!("Handle Error"),
        }
    }
}
