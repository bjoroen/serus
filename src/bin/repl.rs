use std::env;

use lib::vm::VM;

struct REPL {
    vm: VM,
}

impl REPL {
    pub fn new(vm: VM) -> Self {
        Self { vm }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
}
