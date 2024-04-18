use std::fmt::Display;

use crate::instruction::Opcode;

mod lexer;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode, line: usize },
    Register { register: String },
    IntOperand { operand: usize },
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Op { code, line } => write!(f, "{}:{} ", line, code),
            Token::Register { register } => write!(f, "{} ", register),
            Token::IntOperand { operand } => write!(f, "{} ", operand),
        }
    }
}
