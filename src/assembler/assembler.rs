use std::collections::HashMap;

use crate::assembler::parser::Parser;

use super::assembler_instruction::{AssemblerInstruction, AssemblerToken};

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerPhase {
    PhaseOne,
    PhaseTwo,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    name: String,
    instruction: AssemblerInstruction,
    symbol_type: SymbolType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolType {
    Label,
}

impl Symbol {
    pub fn new(name: String, instruction: AssemblerInstruction, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            instruction,
            symbol_type,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.entry(s.name.clone()).or_insert(s);
    }

    pub fn get_symbol(&mut self, key: String) -> Option<&Symbol> {
        let symbol = self.symbols.get(&key);
        symbol
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
        for instruction in program {
            match instruction {
                AssemblerToken::LabelDeclaration {
                    label_name: name,
                    assembler_instruction: instruction,
                } => {
                    let symbol =
                        Symbol::new(String::from(name), instruction.clone(), SymbolType::Label);
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

    use crate::assembler::Token;

    use super::*;

    #[test]
    fn test_symbol_assembler() {
        let mut assembler = Assembler::new();

        assembler.assemble("LOAD $0 #10\nLOAD $1 #10\nmy_label: ADD $2 $0 $1");

        let instruction = AssemblerInstruction {
            opcode: Some(Token::Op {
                code: crate::instruction::Opcode::ADD,
            }),
            directive: None,
            label: None,
            operand_one: Some(Token::Register { register: 2 }),
            operand_two: Some(Token::Register { register: 0 }),
            operand_three: Some(Token::Register { register: 1 }),
        };

        let symbol = Symbol::new(String::from("my_label"), instruction, SymbolType::Label);

        assert_eq!(
            assembler.symbols.get_symbol(String::from("my_label")),
            Some(&symbol)
        );
    }

    #[test]
    fn test_symbol_table() {
        let mut symbol_tabel = SymbolTable::new();
        let instruction = AssemblerInstruction {
            opcode: Some(Token::Op {
                code: crate::instruction::Opcode::JMP,
            }),
            directive: None,
            label: Some(Token::Label {
                name: String::from("test"),
            }),
            operand_one: None,
            operand_two: None,
            operand_three: None,
        };

        let symbol = Symbol::new(String::from("test_label"), instruction, SymbolType::Label);
        symbol_tabel.add_symbol(symbol.clone());

        assert_eq!(symbol_tabel.symbols.len(), 1);
        assert_eq!(
            symbol_tabel.get_symbol(String::from("test_label")),
            Some(&symbol)
        );
    }
}
