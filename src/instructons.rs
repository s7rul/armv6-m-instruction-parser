#[derive(Debug)]
pub enum Instruction {
    Bit16(Instruction16Bit),
    Bit32(Instruction32Bit),
}

impl Instruction {
    pub fn is_16bit(&self) -> bool {
        matches!(self, Instruction::Bit16(_))
    }

    pub fn is_32bit(&self) -> bool {
        matches!(self, Instruction::Bit32(_))
    }
}

#[derive(Debug)]
pub enum Instruction16Bit {
    NOP,
}

#[derive(Debug)]
pub enum Instruction32Bit {
    MSR,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn instruction_size() {
        let instruction_32 = Instruction::Bit32(Instruction32Bit::MSR);
        assert_eq!(instruction_32.is_32bit(), true);
        assert_eq!(instruction_32.is_16bit(), false);

        let instruction_16 = Instruction::Bit16(Instruction16Bit::NOP);
        assert_eq!(instruction_16.is_32bit(), false);
        assert_eq!(instruction_16.is_16bit(), true);
    }
}
