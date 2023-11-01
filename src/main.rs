mod bytecode;
mod vm;

use bytecode::{Instruction};
use vm::SmolVm;

fn main() {
    let code = vec![
        Instruction::ICONST(99),
        Instruction::PRINT,
        Instruction::HALT,
    ];
    let mut vm = SmolVm::new(code, 0, 0, 100);
    let return_code = vm.exec();
    println!("{}", return_code);
}
