use core::panic;

use crate::{assembler::parser::Parser, instruction};

use super::{
    assembler_instruction::{AssemblerInstruction, AssemblerToken},
    symbol::{Symbol, SymbolTable, SymbolType},
};

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerPhase {
    PhaseOne,
    PhaseTwo,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
    pub sections: Vec<String>,
    pub read_only_data: Vec<u8>,
    pub const_offset: u32,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::PhaseOne,
            symbols: SymbolTable::new(),
            sections: vec![],
            read_only_data: vec![],
            const_offset: 0,
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        let mut p = Parser::new(raw);
        p.parse();

        self.first_phase(&p.program);
        Some(self.second_phase(&p.program))
    }

    fn first_phase(&mut self, program: &Vec<AssemblerToken>) {
        self.create_symbol_table(program);
        self.phase = AssemblerPhase::PhaseTwo;
    }

    fn create_symbol_table(&mut self, program: &Vec<AssemblerToken>) {
        for i in program {
            match i {
                AssemblerToken::LabelDeclaration {
                    label_name: name,
                    assembler_instruction: instruction,
                } => {
                    let symbol = Symbol::new(
                        String::from(name),
                        instruction.to_bytes(),
                        SymbolType::Label,
                    );

                    self.symbols.add_symbol(symbol);

                    if instruction.is_directive() {
                        self.process_directive(instruction)
                    }
                }
                AssemblerToken::Instruction {
                    assembler_instruction: instruction,
                } => {
                    if instruction.is_directive() && !instruction.has_operands() {
                        if let Some(directive) = &instruction.directive {
                            self.sections.push(directive.to_string())
                        }
                    }
                }
            }
        }
    }

    fn second_phase(&self, program: &[AssemblerToken]) -> Vec<u8> {
        vec![00, 21, 21]
    }

    fn process_directive(&mut self, instruction: &AssemblerInstruction) {
        if let Some(name) = instruction.get_directive_name() {
            match name {
                ".asciiz" => self.handle_ascii(instruction),
                _ => panic!("Unknown directive"),
            }
        }
    }

    fn handle_ascii(&mut self, i: &AssemblerInstruction) {
        if let Some(s) = i.get_string_content() {
            for byte in s.as_bytes() {
                self.read_only_data.push(*byte);
                self.const_offset += 1
            }

            self.read_only_data.push(0);
            self.const_offset += 1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assembler::assembler::Assembler;

    #[test]
    fn test_symbol_assembler() {
        let mut assembler = Assembler::new();

        assembler.assemble("LOAD $0 #10\nLOAD $1 #10\nmy_label: ADD $2 $0 $1");

        let instruction: Vec<u8> = vec![01, 02, 00, 01];

        assert_eq!(
            assembler.symbols.get_symbol(String::from("my_label")),
            Some(&instruction)
        );
    }

    #[test]
    fn test_sections() {
        let mut assembler = Assembler::new();

        assembler.assemble(".data\n.code\nLOAD $1 #10");

        assert_eq!(
            assembler.sections,
            vec![String::from(".data"), String::from(".code")]
        );
    }

    #[test]
    fn test_read_only_data() {
        let mut assembler = Assembler::new();

        assembler.assemble(".data\nmy_string: .asciiz \"Hello world\"");

        assert_eq!(assembler.read_only_data.len(), "Hello world".len() + 1);
    }
}
