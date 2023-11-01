pub enum Instruction {
    /// Integer add instruction
    IADD,

    /// Integer subtraction instruction.
    ISUB,

    /// Integer multiplication instruction.
    IMUL,

    /// Integer less than test with the top of the stack.
    ILT(i32),

    /// Integer equality test with the top of the stack.
    IEQ(i32),

    /// Load integer into the stack.
    ICONST(i32),

    /// Load value from the memory to the stack.
    MLOAD(usize),

    /// Store value from the top of the stack to the memory.
    MSTORE(usize),

    /// Prints the top of the stack to the standard output.
    PRINT,

    /// Pop value from the stack and burns it.
    POP,

    /// Halt the program.
    HALT,
}