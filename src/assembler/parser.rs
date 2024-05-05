#![allow(dead_code)]

use super::{
    assembler_instruction::{AssemblerInstruction, AssemblerToken},
    lexer::Lexer,
    Token,
};

pub struct Parser {
    lexer: Lexer,
    label: Option<Token>,
    current: Token,
    peek: Token,
    pub program: Vec<AssemblerToken>,
}

impl Parser {
    pub fn new(source_code: &str) -> Parser {
        let lexer = Lexer::new(source_code);
        Parser {
            lexer,
            label: None,
            current: Token::EOF,
            peek: Token::EOF,
            program: vec![],
        }
    }

    pub fn parse(&mut self) {
        self.read();
        self.read();

        while let Some(instruction) = self.next() {
            self.program.push(instruction);
        }
    }

    fn parse_instruction(&mut self) -> Option<AssemblerToken> {
        match &self.current.clone() {
            Token::Op { code: _ } => Some(AssemblerToken::Instruction {
                assembler_instruction: self.parse_opcode_instruction(),
            }),
            Token::Directive { value: _ } => Some(AssemblerToken::Instruction {
                assembler_instruction: self.parse_directive_instruction(),
            }),
            Token::LabelDeclaration { value: v } => {
                self.read();

                let token_type = match &self.current {
                    Token::Op { code: _ } => self.parse_opcode_instruction(),
                    Token::Directive { value: _ } => self.parse_directive_instruction(),
                    Token::EOF => todo!(),
                    _ => todo!(),
                };

                Some(AssemblerToken::LabelDeclaration {
                    label_name: String::from(v),
                    assembler_instruction: token_type,
                })
            }
            Token::EOF => None,
            _ => panic!("Unexpected Token, instruction has to start with Opcode, Directive or LabelDeclaration"),
        }
    }

    fn parse_opcode_instruction(&mut self) -> AssemblerInstruction {
        let op = self.current.clone();

        // Eat the OP token
        self.read();

        let label = match &self.current {
            Token::Label { name: _ } => {
                let label = self.current.clone();
                self.read();
                Some(label)
            }
            _ => None,
        };

        AssemblerInstruction {
            opcode: Some(op),
            directive: None,
            label,
            operand_one: self.next_operand(),
            operand_two: self.next_operand(),
            operand_three: self.next_operand(),
        }
    }

    fn parse_directive_instruction(&mut self) -> AssemblerInstruction {
        let dir = self.current.clone();
        // eat the Directive token
        self.read();

        let label = match self.peek {
            Token::Label { name: _ } => {
                let label = self.current.clone();
                self.read();
                Some(label)
            }
            _ => None,
        };

        AssemblerInstruction {
            opcode: None,
            directive: Some(dir),
            label,
            operand_one: self.next_operand(),
            operand_two: self.next_operand(),
            operand_three: self.next_operand(),
        }
    }

    fn next_operand(&mut self) -> Option<Token> {
        match self.current {
            Token::Register { register: _ } => {
                let current = self.current.clone();
                self.read();

                Some(current)
            }
            Token::StringOperand { operand: _ } => {
                let current = self.current.clone();
                self.read();

                Some(current)
            }
            Token::IntOperand { operand: _ } => {
                let current = self.current.clone();
                self.read();

                Some(current)
            }
            _ => None,
        }
    }

    fn next(&mut self) -> Option<AssemblerToken> {
        if self.current == Token::EOF {
            return None;
        }

        self.label = None;
        self.parse_instruction()
    }

    /// Reads and eats the next token
    pub fn read(&mut self) {
        self.current = self.peek.clone();
        self.peek = if let Some(token) = self.lexer.next() {
            token
        } else {
            Token::EOF
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_label() {
        let mut p = Parser::new("JMP @test");
        p.parse();

        assert_eq!(p.program.len(), 1);

        assert_eq!(
            p.program[0],
            AssemblerToken::Instruction {
                assembler_instruction: AssemblerInstruction {
                    opcode: Some(Token::Op {
                        code: crate::instruction::Opcode::JMP
                    }),
                    directive: None,
                    label: Some(Token::Label {
                        name: String::from("test")
                    }),
                    operand_one: None,
                    operand_two: None,
                    operand_three: None
                }
            }
        );
    }

    #[test]
    fn test_parse_label_declaration() {
        let mut p = Parser::new("my_instruction: load $10 #10");
        p.parse();

        assert_eq!(p.program.len(), 1);

        assert_eq!(
            p.program[0],
            AssemblerToken::LabelDeclaration {
                label_name: String::from("my_instruction"),
                assembler_instruction: AssemblerInstruction {
                    opcode: Some(Token::Op {
                        code: crate::instruction::Opcode::LOAD
                    }),
                    directive: None,
                    label: None,
                    operand_one: Some(Token::Register { register: 10 }),
                    operand_two: Some(Token::IntOperand { operand: 10 }),
                    operand_three: None
                }
            }
        );
    }

    #[test]
    fn test_parse_instruction() {
        let mut p =
            Parser::new("my_string: .asciiz \"Hello world\"\nload $10 #10\nHLT\nADD $0 $10 $5");
        p.parse();

        assert_eq!(p.program.len(), 4);

        assert_eq!(
            p.program[0],
            AssemblerToken::LabelDeclaration {
                label_name: String::from("my_string"),
                assembler_instruction: AssemblerInstruction {
                    opcode: None,
                    directive: Some(Token::Directive {
                        value: String::from("asciiz")
                    }),
                    label: None,
                    operand_one: Some(Token::StringOperand {
                        operand: String::from("Hello world")
                    }),
                    operand_two: None,
                    operand_three: None
                }
            }
        );

        assert_eq!(
            p.program[1],
            AssemblerToken::Instruction {
                assembler_instruction: AssemblerInstruction {
                    opcode: Some(Token::Op {
                        code: crate::instruction::Opcode::LOAD
                    }),
                    directive: None,
                    label: None,
                    operand_one: Some(Token::Register { register: 10 }),
                    operand_two: Some(Token::IntOperand { operand: 10 }),
                    operand_three: None
                }
            }
        );

        assert_eq!(
            p.program[2],
            AssemblerToken::Instruction {
                assembler_instruction: AssemblerInstruction {
                    opcode: Some(Token::Op {
                        code: crate::instruction::Opcode::HLT
                    }),
                    directive: None,
                    label: None,
                    operand_one: None,
                    operand_two: None,
                    operand_three: None
                }
            }
        );

        assert_eq!(
            p.program[3],
            AssemblerToken::Instruction {
                assembler_instruction: AssemblerInstruction {
                    opcode: Some(Token::Op {
                        code: crate::instruction::Opcode::ADD
                    }),
                    directive: None,
                    label: None,
                    operand_one: Some(Token::Register { register: 0 }),
                    operand_two: Some(Token::Register { register: 10 }),
                    operand_three: Some(Token::Register { register: 5 }),
                }
            }
        )
    }
}
