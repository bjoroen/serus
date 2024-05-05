use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    name: String,
    instruction: Vec<u8>,
    symbol_type: SymbolType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolType {
    Label,
}

impl Symbol {
    pub fn new(name: String, instruction: Vec<u8>, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            instruction,
            symbol_type,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Vec<u8>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.entry(s.name.clone()).or_insert(s.instruction);
    }

    pub fn get_symbol(&mut self, key: String) -> Option<&Vec<u8>> {
        let symbol = self.symbols.get(&key);
        symbol
    }
}

#[cfg(test)]
mod tests {
    use crate::assembler::{
        assembler_instruction::AssemblerInstruction,
        symbol::{Symbol, SymbolTable, SymbolType},
        Token,
    };

    #[test]
    fn test_symbol_table() {
        let mut symbol_tabel = SymbolTable::new();
        let instruction = AssemblerInstruction {
            opcode: Some(Token::Op {
                code: crate::instruction::Opcode::JMP,
            }),
            directive: None,
            label: Some(Token::Label {
                name: String::from("test_label"),
            }),
            operand_one: None,
            operand_two: None,
            operand_three: None,
        };

        let symbol = Symbol::new(
            String::from("test_label"),
            instruction.to_bytes(),
            SymbolType::Label,
        );
        symbol_tabel.add_symbol(symbol.clone());

        let instruction: Vec<u8> = vec![06, 00, 00, 00];

        assert_eq!(symbol_tabel.symbols.len(), 1);
        assert_eq!(
            symbol_tabel.get_symbol(String::from("test_label")),
            Some(&instruction)
        );
    }
}
