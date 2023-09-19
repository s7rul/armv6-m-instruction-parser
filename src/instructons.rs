#[derive(Debug)]
pub enum Instruction {
    Bit16(Instruction16Bit),
    Bit32(Instruction32Bit),
}

#[derive(Debug)]
pub enum Instruction16Bit {
    NOP,
}

#[derive(Debug)]
pub enum Instruction32Bit {
    NOP,
}
