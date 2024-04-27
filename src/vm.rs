#![allow(dead_code)]

use crate::instruction::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
    heap: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            heap: vec![],
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
            return true;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bites_usize();
                let number = self.next_16_bits_usize();

                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let load_register = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                self.registers[load_register] = register_one + register_two;
            }
            Opcode::MUL => {
                let load_register = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                self.registers[load_register] = register_one * register_two;
            }
            Opcode::SUB => {
                let load_register = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                self.registers[load_register] = register_one - register_two;
            }
            Opcode::DIV => {
                let load_register = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                self.registers[load_register] = register_one / register_two;
                self.remainder = (register_one % register_two) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bites_usize()];
                self.pc = target as usize;
            }
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bites_usize()];
                self.pc -= target as usize
            }
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bites_usize()];
                self.pc += target as usize
            }
            Opcode::EQ => {
                let target = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                match register_one == register_two {
                    true => self.registers[target] = 1,
                    false => self.registers[target] = 0,
                }
            }
            Opcode::NEQ => {
                let target = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                match register_one == register_two {
                    true => self.registers[target] = 0,
                    false => self.registers[target] = 1,
                }
            }
            Opcode::GT => {
                let target = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                match register_one > register_two {
                    true => self.registers[target] = 1,
                    false => self.registers[target] = 0,
                }
            }
            Opcode::LT => {
                let target = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                match register_one < register_two {
                    true => self.registers[target] = 1,
                    false => self.registers[target] = 0,
                }
            }
            Opcode::GTQ => {
                let target = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                match register_one >= register_two {
                    true => self.registers[target] = 1,
                    false => self.registers[target] = 0,
                }
            }
            Opcode::LTQ => {
                let target = self.next_8_bites_usize();
                let register_one = self.registers[self.next_8_bites_usize()];
                let register_two = self.registers[self.next_8_bites_usize()];

                match register_one <= register_two {
                    true => self.registers[target] = 1,
                    false => self.registers[target] = 0,
                }
            }
            Opcode::JEQ => {
                let target = self.next_8_bites_usize();
                let bool_register = self.next_8_bites_usize();

                if self.registers[bool_register] == 1 {
                    self.pc = self.registers[target] as usize;
                }
            }
            Opcode::JNEQ => {
                let target = self.next_8_bites_usize();
                let bool_register = self.next_8_bites_usize();

                if self.registers[bool_register] == 0 {
                    self.pc = self.registers[target] as usize;
                }
            }
            Opcode::ALOC => {
                let register = self.next_8_bites_usize();
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            Opcode::HLT => {
                println!("HLT encountered");
                true;
            }
            Opcode::IGL => {
                println!("IGL encountered");
                true;
            }
        }

        false
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

    fn next_8_bites_usize(&mut self) -> usize {
        self.next_8_bits() as usize
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;

        result
    }

    fn next_16_bits_usize(&mut self) -> usize {
        self.next_16_bits() as usize
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

        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![254, 0, 0, 0];
        test_vm.program = test_bytes;

        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244];

        test_vm.run();
        assert_eq!(test_vm.registers[0], 500)
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 500;
        test_vm.registers[2] = 500;
        test_vm.program = vec![1, 0, 1, 2];

        test_vm.run();
        assert_eq!(test_vm.registers[0], 1000)
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 8;
        test_vm.registers[2] = 5;
        test_vm.program = vec![2, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 1);
        assert_eq!(test_vm.remainder, 3);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 5;
        test_vm.program = vec![3, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 10)
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 5;
        test_vm.registers[2] = 2;
        test_vm.program = vec![4, 0, 1, 2];

        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 3)
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmp_back() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_jmp_forward() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![8, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();

        // Checks if equal - should return 1 (true)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 2;
        test_vm.program = vec![9, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        // Reset program counter
        test_vm.pc = 0;

        // Checks if is equal - should return 0 (false)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 3;
        test_vm.program = vec![9, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::new();

        // Checks if not equal - should return 1 (true)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 3;
        test_vm.program = vec![10, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        // Reset program counter
        test_vm.pc = 0;

        // Checks if is equal - should return 0 (false)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 2;
        test_vm.program = vec![10, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::new();

        // Checks if greater than - should return 1 (true)
        test_vm.registers[1] = 3;
        test_vm.registers[2] = 2;
        test_vm.program = vec![11, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        // Reset program counter
        test_vm.pc = 0;

        // Checks if not greater than - should return 0 (false)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 3;
        test_vm.program = vec![11, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::new();

        // Checks if less than - should return 1 (true)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 3;
        test_vm.program = vec![12, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        // Reset program counter
        test_vm.pc = 0;

        // Checks if not less than - should return 0 (false)
        test_vm.registers[1] = 3;
        test_vm.registers[2] = 2;
        test_vm.program = vec![12, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_gtq() {
        let mut test_vm = VM::new();

        // Checks if equal - should return 1 (true)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 2;
        test_vm.program = vec![13, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        // Reset program counter
        test_vm.pc = 0;

        // Checks if greater than - should return 1 (true)
        test_vm.registers[1] = 3;
        test_vm.registers[2] = 2;
        test_vm.program = vec![13, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        // Reset program counter
        test_vm.pc = 0;

        // Checks if not greater than - should return 0 (false)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 3;
        test_vm.program = vec![13, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_ltq() {
        let mut test_vm = VM::new();

        // Checks if equal - should return 1 (true)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 2;
        test_vm.program = vec![14, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        // Reset program counter
        test_vm.pc = 0;

        // Checks if greater than - should return 1 (true)
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 3;
        test_vm.program = vec![14, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 1);

        // Reset program counter
        test_vm.pc = 0;

        // Checks if not greater than - should return 0 (false)
        test_vm.registers[1] = 3;
        test_vm.registers[2] = 2;
        test_vm.program = vec![14, 0, 1, 2];
        test_vm.run_once();

        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 2;
        test_vm.registers[1] = 1;
        test_vm.program = vec![15, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 2);
    }

    #[test]
    fn test_opcode_jneq() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 2;
        test_vm.registers[1] = 0;
        test_vm.program = vec![16, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 2);
    }

    #[test]
    fn test_opcode_aloc() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 256;
        test_vm.program = vec![17, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.heap.len(), 256);
    }
}
