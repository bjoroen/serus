#![allow(dead_code)]

use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
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
    ALOC,
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
            17 => Opcode::ALOC,
            _ => Opcode::IGL,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> u8 {
        match op {
            Opcode::LOAD => 0,
            Opcode::ADD => 1,
            Opcode::DIV => 2,
            Opcode::MUL => 3,
            Opcode::SUB => 4,
            Opcode::HLT => 5,
            Opcode::JMP => 6,
            Opcode::JMPB => 7,
            Opcode::JMPF => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GT => 11,
            Opcode::LT => 12,
            Opcode::GTQ => 13,
            Opcode::LTQ => 14,
            Opcode::JEQ => 15,
            Opcode::JNEQ => 16,
            Opcode::ALOC => 17,
            Opcode::IGL => 100,
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::LOAD => write!(f, "load"),
            Opcode::ADD => write!(f, "add"),
            Opcode::DIV => write!(f, "div"),
            Opcode::MUL => write!(f, "mul"),
            Opcode::SUB => write!(f, "sub"),
            Opcode::HLT => write!(f, "hlt"),
            Opcode::JMP => write!(f, "jmp"),
            Opcode::JMPB => write!(f, "jmpb"),
            Opcode::JMPF => write!(f, "jmpf"),
            Opcode::EQ => write!(f, "eq"),
            Opcode::NEQ => write!(f, "neq"),
            Opcode::GT => write!(f, "gt"),
            Opcode::LT => write!(f, "lt"),
            Opcode::GTQ => write!(f, "gtq"),
            Opcode::LTQ => write!(f, "ltq"),
            Opcode::JEQ => write!(f, "jeq"),
            Opcode::JNEQ => write!(f, "jneq"),
            Opcode::ALOC => write!(f, "aloc"),
            Opcode::IGL => write!(f, "igl"),
        }
    }
}

impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "load" => Ok(Opcode::LOAD),
            "add" => Ok(Opcode::ADD),
            "div" => Ok(Opcode::DIV),
            "sub" => Ok(Opcode::SUB),
            "hlt" => Ok(Opcode::HLT),
            "jmp" => Ok(Opcode::JMP),
            "jmpb" => Ok(Opcode::JMPB),
            "jmpf" => Ok(Opcode::JMPF),
            "eq" => Ok(Opcode::EQ),
            "NEQ" => Ok(Opcode::NEQ),
            "gt" => Ok(Opcode::GT),
            "lq" => Ok(Opcode::LT),
            "gtq" => Ok(Opcode::GTQ),
            "ltq" => Ok(Opcode::LTQ),
            "jeq" => Ok(Opcode::JEQ),
            "jneq" => Ok(Opcode::JNEQ),
            _ => Err(()),
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
