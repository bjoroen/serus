#![allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Opcode {
    LOAD,
    ADD,
    DIV,
    MUL,
    SUB,
    HLT,
    JMP,
    IGL,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::DIV,
            3 => Opcode::MUL,
            4 => Opcode::SUB,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    use super::Opcode;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
