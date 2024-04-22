use std::{io::stdin, process::exit};

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

            if buffer.starts_with("!") {
                self.run_command(&buffer)
            } else {
                let mut program = Program::parse_program(&buffer);
                self.vm.program.append(&mut program);
                self.vm.run_once();
                dbg!("{}", self.vm.registers);
            }
        }
    }

    fn run_command(&self, input: &str) {
        match input {
            "!Q" => self.quit(),
            _ => println!("{input}"),
        }
    }

    fn quit(&self) {
        println!("Good bye, happy coding! :D");
        exit(0)
    }
}

fn main() {
    let vm = VM::new();
    let mut repl = REPL::new(vm);

    repl.run();
}
