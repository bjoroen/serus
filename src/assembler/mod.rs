use std::fmt::Display;

use crate::instruction::Opcode;

pub mod assembler;
mod assembler_instruction;
mod lexer;
mod parser;
pub mod program;
mod symbol;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op { code: Opcode },
    Register { register: i32 },
    IntOperand { operand: i32 },
    StringOperand { operand: String },
    LabelDeclaration { value: String },
    Label { name: String },
    Directive { value: String },
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Op { code } => write!(f, "{} ", code),
            Token::Register { register } => write!(f, "{} ", register),
            Token::IntOperand { operand } => write!(f, "{} ", operand),
            Token::StringOperand { operand } => write!(f, "{}", operand),
            Token::LabelDeclaration { value } => write!(f, "{} ", value),
            Token::Label { name } => write!(f, "{} ", name),
            Token::Directive { value } => write!(f, ".{}", value),
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
            Token::EOF => todo!("Handle Error"),
            // TODO: Figure out how to handle from and to u8 for Labels and Directive
            _ => todo!(),
        }
    }
}
