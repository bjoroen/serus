use crate::assembler::parser::Parser;

use super::{
    assembler_instruction::AssemblerToken,
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
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::PhaseOne,
            symbols: SymbolTable::new(),
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
        for instruction in program {
            match instruction {
                AssemblerToken::LabelDeclaration {
                    label_name: name,
                    assembler_instruction: instruction,
                } => {
                    let symbol = Symbol::new(
                        String::from(name),
                        instruction.to_bytes(),
                        SymbolType::Label,
                    );
                    self.symbols.add_symbol(symbol)
                }
                AssemblerToken::Instruction {
                    assembler_instruction: _,
                } => {}
            }
        }
    }

    fn second_phase(&self, program: &[AssemblerToken]) -> Vec<u8> {
        vec![00, 21, 21]
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
}
