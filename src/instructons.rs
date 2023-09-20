#[derive(Debug)]
pub struct Instruction {
    pub width: InstructionWidth,
    pub operation: Operation,
}

#[derive(Debug)]
pub enum InstructionWidth {
    Bit32,
    Bit16,
}

impl Instruction {
    pub fn is_16bit(&self) -> bool {
        matches!(self.width, InstructionWidth::Bit16)
    }

    pub fn is_32bit(&self) -> bool {
        matches!(self.width, InstructionWidth::Bit32)
    }
}

#[derive(Debug)]
pub enum Operation {
    NOP,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn instruction_size() {
        let instruction_32 = Instruction {
            width: InstructionWidth::Bit32,
            operation: Operation::NOP,
        };
        assert_eq!(instruction_32.is_32bit(), true);
        assert_eq!(instruction_32.is_16bit(), false);

        let instruction_16 = Instruction {
            width: InstructionWidth::Bit16,
            operation: Operation::NOP,
        };
        assert_eq!(instruction_16.is_32bit(), false);
        assert_eq!(instruction_16.is_16bit(), true);
    }
}
