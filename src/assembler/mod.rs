use std::fmt::Display;

use crate::instruction::Opcode;

mod lexer;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode, line: usize },
    Register { register: usize, line: usize },
    IntOperand { operand: usize, line: usize },
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Op { code, line } => write!(f, "{}:{}", line, code),
            Token::Register { register, line: _ } => write!(f, "{}", register),
            Token::IntOperand { operand, line: _ } => write!(f, "{}", operand),
        }
    }
}
