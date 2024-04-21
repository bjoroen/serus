use super::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AssemblerInstruction {
    pub opcode: Token,
    pub operand_one: Option<Token>,
    pub operand_two: Option<Token>,
    pub operand_three: Option<Token>,
}
