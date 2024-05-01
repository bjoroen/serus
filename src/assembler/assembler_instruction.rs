use core::panic;

use super::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerToken {
    LabelDeclaration {
        label_name: String,
        assembler_instruction: AssemblerInstruction,
    },
    Instruction {
        assembler_instruction: AssemblerInstruction,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct LabelDeclaration {
    pub assembler_instruction: AssemblerInstruction,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub directive: Option<Token>,
    pub label: Option<Token>,
    pub operand_one: Option<Token>,
    pub operand_two: Option<Token>,
    pub operand_three: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        match &self.opcode {
            Some(Token::Op { code }) => result.push(*code as u8),
            _ => {
                panic!("Expected opcode or Directive")
            }
        }

        for operand in vec![&self.operand_one, &self.operand_two, &self.operand_three] {
            match operand {
                Some(t) => self.get_operand(t, &mut result),
                None => {}
            }
        }

        while result.len() > 4 {
            result.push(0)
        }

        result
    }

    fn get_operand(&self, t: &Token, result: &mut Vec<u8>) {
        match t {
            Token::Register { register } => result.push(*register as u8),
            Token::IntOperand { operand } => {
                let converted = *operand as u16;
                let byte_one = converted;
                let byte_two = converted >> 8;
                result.push(byte_two as u8);
                result.push(byte_one as u8);
            }

            e => {
                panic!("Expected Register or Integer operand, found {e}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemblerinstruction_tobytes() {
        let ai = AssemblerInstruction {
            opcode: Some(Token::Op {
                code: crate::instruction::Opcode::LOAD,
            }),
            operand_one: Some(Token::Register { register: 10 }),
            operand_two: Some(Token::IntOperand { operand: 500 }),
            operand_three: None,
            directive: None,
            label: None,
        };

        assert_eq!(ai.to_bytes(), [0, 10, 1, 244])
    }

    #[test]
    fn test_assemblerinstruction_add_tobytes() {
        let ai = AssemblerInstruction {
            opcode: Some(Token::Op {
                code: crate::instruction::Opcode::ADD,
            }),
            operand_one: Some(Token::Register { register: 0 }),
            operand_two: Some(Token::Register { register: 10 }),
            operand_three: Some(Token::Register { register: 5 }),
            directive: None,
            label: None,
        };

        assert_eq!(ai.to_bytes(), [1, 0, 10, 5])
    }
}
