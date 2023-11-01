use crate::bytecode::Instruction;

pub struct SmolVm {
    /// Program to be executed by the VM.
    code: Vec<Instruction>,
    /// Stack of the VM.
    stack: Vec<i32>,
    /// Memory of the VM.
    data_memory: Vec<i32>,
    /// Instruction pointer.
    ip: usize,
    /// Stack pointer.
    sp: usize,
    /// Frame pointer.
    fp: i32,
}

impl SmolVm {
    pub fn new(code: Vec<Instruction>, init_ip: usize, data_size: usize, stack_size: usize) -> Self {
        Self {
            code: code,
            ip: init_ip,
            data_memory: vec![0; data_size],
            stack: vec![0; stack_size],
            sp: 0,
            fp: 0,
        }
    }

    pub fn exec(&mut self) -> bool {
        // Fetch the instruction
        let mut instruction = &self.code[self.ip];
        while self.ip < self.code.len() {
            self.ip += 1;

            match instruction {
                Instruction::ICONST(value) => {
                    self.sp += 1;
                    self.stack[self.sp] = *value;
                },
                Instruction::PRINT => {
                    let value = self.stack[self.sp];
                    println!("{}", value);
                },
                Instruction::HALT => return true,
                Instruction::IADD => {
                    if self.stack.len() + 1 < 2 {
                        return false;
                    } else {
                        let value1 = self.stack[self.sp];
                        self.sp -= 1;
                        let value2 = self.stack[self.sp];
                        self.sp -= 1;
                        let sum = value1 + value2;
                        self.sp += 1;
                        self.stack[self.sp] = sum;
                    }
                },
                Instruction::MSTORE(addr) => {
                    let value = self.stack[self.sp];
                    self.sp -= 1;
                    self.data_memory[*addr] = value;
                }
                _ => return false,
            }
            instruction = &self.code[self.ip];
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_construction_and_exec() {
        let code = vec![Instruction::HALT];
        let mut vm = SmolVm::new(code, 0, 0, 100);
        let return_code = vm.exec();
        assert!(return_code);
    }

    #[test]
    fn print_elemnt() {
        let code = vec![
            Instruction::ICONST(99),
            Instruction::PRINT,
            Instruction::HALT,
        ];
        let mut vm = SmolVm::new(code, 0, 0, 100);
        let return_code = vm.exec();
        assert!(return_code);
    }
}
