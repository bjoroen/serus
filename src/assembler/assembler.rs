use crate::assembler::parser::Parser;

use super::assembler_instruction::AssemblerToken;

pub enum AssemblerPhase {
    PhaseOne,
    PhaseTwo,
}

pub struct Symbol {}

pub struct SymbolTable {}

pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        todo!()
    }
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
        todo!()
    }

    fn second_phase(&self, program: &[AssemblerToken]) -> Vec<u8> {
        todo!()
    }
}
