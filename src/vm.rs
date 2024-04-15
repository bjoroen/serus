#![allow(dead_code)]

use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;

                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register_one = self.registers[self.next_8_bits() as usize];
                let register_two = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register_one + register_two;
            }
            Opcode::MUL => {
                let register_one = self.registers[self.next_8_bits() as usize];
                let register_two = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register_one * register_two;
            }
            Opcode::SUB => {
                let register_one = self.registers[self.next_8_bits() as usize];
                let register_two = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register_one - register_two;
            }
            Opcode::DIV => {
                let register_one = self.registers[self.next_8_bits() as usize];
                let register_two = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register_one / register_two;
                self.remainder = (register_one % register_two) as u32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                false;
            }
            Opcode::JMP => todo!(),
            Opcode::IGL => {
                println!("IGL encountered");
                false;
            }
        }

        true
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;

        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;

        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;

        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![254, 0, 0, 0];
        test_vm.program = test_bytes;

        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 0xF4];

        test_vm.run();
        assert_eq!(test_vm.registers[0], 500)
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 500;
        test_vm.registers[1] = 500;
        test_vm.program = vec![1, 0, 1, 2];

        test_vm.run();
        assert_eq!(test_vm.registers[2], 1000)
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 8;
        test_vm.registers[1] = 5;
        test_vm.program = vec![2, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 3);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 5;
        test_vm.program = vec![3, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 10)
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 2;
        test_vm.program = vec![4, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 3)
    }

    // Impl Test for JMP
    #[test]
    fn test_opcode_jmp() {
        todo!()
    }
}
