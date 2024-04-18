use crate::instruction::Opcode;

mod lexer;

pub enum Token {
    Op { code: Opcode },
}
