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
    JMPB,
    JMPF,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
    JNEQ,
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
            7 => Opcode::JMPB,
            8 => Opcode::JMPF,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTQ,
            14 => Opcode::LTQ,
            15 => Opcode::JEQ,
            16 => Opcode::JNEQ,
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
