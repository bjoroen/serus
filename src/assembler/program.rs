use super::parser::Parser;

pub struct Program;

impl Program {
    pub fn parse_program(source: &str) -> Vec<u8> {
        let mut program: Vec<u8> = vec![];
        let mut parser = Parser::new(source);
        parser.parse();
        let instructions = parser.program;

        for instruction in &instructions {
            program.append(&mut instruction.to_bytes())
        }

        program
    }
}
