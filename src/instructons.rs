//! Provides a instruction type and a enum with all operations and there arguments.

use crate::{
    conditions::Condition,
    registers::{Register, SpecialRegister},
};

/// Struct describing an instruction.
#[derive(Debug)]
pub struct Instruction {
    pub width: InstructionWidth,
    pub operation: Operation,
}

/// Enum describing the with of the corresponding binary representation of the instruction.
#[derive(Debug)]
pub enum InstructionWidth {
    Bit32,
    Bit16,
}

impl Instruction {
    /// To check if instruction width is 16 bits.
    pub fn is_16bit(&self) -> bool {
        matches!(self.width, InstructionWidth::Bit16)
    }

    /// To check if instruction width is 32 bits.
    pub fn is_32bit(&self) -> bool {
        matches!(self.width, InstructionWidth::Bit32)
    }
}

/// Describes operation i.e. what type of instruction it is.
#[derive(Debug)]
pub enum Operation {
    ADCReg {
        m: Register,
        n: Register,
        d: Register,
    },
    ADDImm {
        imm: u32,
        n: Register,
        d: Register,
    },
    ADDReg {
        m: Register,
        n: Register,
        d: Register,
    },
    ADDImmSP {
        d: Register,
        imm: u32,
    },
    ADDRegSP {
        d: Register,
        m: Register,
    },
    ADR {
        d: Register,
        imm: u32,
    },
    ANDReg {
        m: Register,
        dn: Register,
    },
    ASRImm {
        imm: u32,
        m: Register,
        d: Register,
    },
    ASRReg {
        m: Register,
        dn: Register,
    },
    B {
        cond: Condition,
        imm: u32,
    },
    BICReg {
        m: Register,
        dn: Register,
    },
    BKPT {
        imm: u32,
    },
    BL {
        imm: u32,
    },
    BLXReg {
        m: Register,
    },
    BX {
        m: Register,
    },
    CMNReg {
        m: Register,
        n: Register,
    },
    CMPImm {
        n: Register,
        imm: u32,
    },
    CMPReg {
        m: Register,
        n: Register,
    },
    CPS {
        im: bool,
    },
    CPY,
    DMB {
        option: u8,
    },
    DSB {
        option: u8,
    },
    EORReg {
        m: Register,
        dn: Register,
    },
    ISB {
        option: u8,
    },
    LDM {
        n: Register,
        reg_list: Vec<Register>,
    },
    LDRImm {
        imm: u32,
        n: Register,
        t: Register,
    },
    LDRLiteral {
        t: Register,
        imm: u32,
    },
    LDRReg {
        m: Register,
        n: Register,
        t: Register,
    },
    LDRBImm {
        imm: u32,
        n: Register,
        t: Register,
    },
    LDRBReg {
        m: Register,
        n: Register,
        t: Register,
    },
    LDRHImm {
        imm: u32,
        n: Register,
        t: Register,
    },
    LDRHReg {
        m: Register,
        n: Register,
        t: Register,
    },
    LDRSBReg {
        m: Register,
        n: Register,
        t: Register,
    },
    LDRSH {
        m: Register,
        n: Register,
        t: Register,
    },
    LSLImm {
        imm: u32,
        m: Register,
        d: Register,
    },
    LSLReg {
        m: Register,
        dn: Register,
    },
    LSRImm {
        imm: u32,
        m: Register,
        d: Register,
    },
    LSRReg {
        m: Register,
        dn: Register,
    },
    MOVImm {
        d: Register,
        imm: u32,
    },
    MOVReg {
        m: Register,
        d: Register,
        set_flags: bool,
    },
    MRS {
        d: Register,
        sysm: SpecialRegister,
    },
    MSRReg {
        n: Register,
        sysm: SpecialRegister,
    },
    MUL {
        n: Register,
        dm: Register,
    },
    MVNReg {
        m: Register,
        d: Register,
    },
    NOP,
    ORRReg {
        m: Register,
        dn: Register,
    },
    POP {
        reg_list: Vec<Register>,
    },
    PUSH {
        reg_list: Vec<Register>,
    },
    REV {
        m: Register,
        d: Register,
    },
    REV16 {
        m: Register,
        d: Register,
    },
    REVSH {
        m: Register,
        d: Register,
    },
    RORReg {
        m: Register,
        dn: Register,
    },
    RSBImm {
        n: Register,
        d: Register,
    },
    SBCReg {
        m: Register,
        dn: Register,
    },
    SEV,
    STM {
        n: Register,
        reg_list: Vec<Register>,
    },
    STRImm {
        imm: u32,
        n: Register,
        t: Register,
    },
    STRReg {
        m: Register,
        n: Register,
        t: Register,
    },
    STRBImm {
        imm: u32,
        n: Register,
        t: Register,
    },
    STRBReg {
        m: Register,
        n: Register,
        t: Register,
    },
    STRHImm {
        imm: u32,
        n: Register,
        t: Register,
    },
    STRHReg {
        m: Register,
        n: Register,
        t: Register,
    },
    SUBImm {
        imm: u32,
        n: Register,
        d: Register,
    },
    SUBReg {
        m: Register,
        n: Register,
        d: Register,
    },
    SUBImmSP {
        imm: u32,
    },
    SVC {
        imm: u32,
    },
    SXTB {
        m: Register,
        d: Register,
    },
    SXTH {
        m: Register,
        d: Register,
    },
    TSTReg {
        m: Register,
        n: Register,
    },
    UDFT1 {
        imm: u32,
    },
    UDFT2 {
        imm: u32,
    },
    UXTB {
        m: Register,
        d: Register,
    },
    UXTH {
        m: Register,
        d: Register,
    },
    WFE,
    WFI,
    YIELD,
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
