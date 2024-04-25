use std::io::{stdin, stdout, Write};

use lib::{assembler::program::Program, vm::VM};

struct REPL {
    vm: VM,
}

impl REPL {
    pub fn new(vm: VM) -> Self {
        Self { vm }
    }

    pub fn run(&mut self) {
        loop {
            let mut buffer = String::new();

            if let Err(e) = stdin().read_line(&mut buffer) {
                eprintln!("Could not read from stdin: {e}");
                break;
            }
            if buffer.starts_with(":") {
                self.run_command(&buffer)
            } else {
                let mut program = Program::parse_program(&buffer);
                self.vm.program.append(&mut program);
                self.vm.run_once();
            }
        }
    }

    fn run_command(&self, input: &str) {
        match input.trim_end() {
            ":quit" | ":q" => self.quit(),
            ":registers" | ":r" => self.show_registers(),
            ":program" | ":p" => self.show_program(),
            _ => println!("{input}"),
        }
    }

    fn quit(&self) {
        println!("Good bye, happy coding! :D");
        std::process::exit(0)
    }

    fn show_registers(&self) {
        for (index, register) in self.vm.registers.iter().enumerate() {
            println!("Register: {} -> {}", index, register)
        }
    }

    fn show_program(&self) {
        let chunks: Vec<&[u8]> = self.vm.program.chunks(4).collect();
        for chunk in chunks {
            chunk.iter().for_each(|c| print!("{}", c));
            println!();
            println!("-------------");
        }
    }
}

fn main() {
    let vm = VM::new();
    let mut repl = REPL::new(vm);

    repl.run();
}
