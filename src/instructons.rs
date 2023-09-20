use crate::{
    conditions::Condition,
    registers::{Register, SpecialRegister},
};

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
    ADCReg {
        rm: Register,
        rdn: Register,
    },
    ADDImmT1 {
        imm: u32,
        rn: Register,
        rd: Register,
    },
    ADDImmT2 {
        imm: u32,
        rdn: Register,
    },
    ADDRegT1 {
        rm: Register,
        rn: Register,
        rd: Register,
    },
    ADDRegT2 {
        rm: Register,
        rdn: Register,
    },
    ADDImmSPT1 {
        rd: Register,
        imm: u32,
    },
    ADDImmSPT2 {
        imm: u32,
    },
    ADDRegSPT1 {
        rdm: Register,
    },
    ADDRegSPT2 {
        rm: Register,
    },
    ADR {
        rd: Register,
        imm: u32,
    },
    ANDReg {
        rm: Register,
        rdn: Register,
    },
    ASRImm {
        imm: u32,
        rm: Register,
        rd: Register,
    },
    ASRReg {
        rm: Register,
        rdn: Register,
    },
    BT1 {
        cond: Condition,
        imm: u32,
    },
    BT2 {
        imm: u32,
    },
    BICReg {
        rm: Register,
        rdn: Register,
    },
    BKPT {
        imm: u32,
    },
    BL {
        imm: u32,
    },
    BLXReg {
        rm: Register,
    },
    BX {
        rm: Register,
    },
    CMNReg {
        rm: Register,
        rn: Register,
    },
    CMPImm {
        rn: Register,
        imm: u32,
    },
    CMPRegT1 {
        rm: Register,
        rn: Register,
    },
    CMPRegT2 {
        rm: Register,
        rn: Register,
    },
    CPS,
    CPY,
    CMB {
        option: u8,
    },
    DSB {
        option: u8,
    },
    EORReg {
        rm: Register,
        rdn: Register,
    },
    ISB {
        option: u8,
    },
    LDM {
        rn: Register,
        reg_list: Vec<Register>,
    },
    LDRImmT1 {
        imm: u32,
        rn: Register,
        rt: Register,
    },
    LDRImmT2 {
        rt: Register,
        imm: Register,
    },
    LDRLiteral {
        rt: Register,
        imm: Register,
    },
    LDRReg {
        rm: Register,
        rn: Register,
        rt: Register,
    },
    LDRBImm {
        imm: u32,
        rn: Register,
        rt: Register,
    },
    LDRBReg {
        rm: Register,
        rn: Register,
        rt: Register,
    },
    LDRHImm {
        imm: u32,
        rn: Register,
        rt: Register,
    },
    LDRHReg {
        rm: Register,
        rn: Register,
        rt: Register,
    },
    LDRSBReg {
        rm: Register,
        rn: Register,
        rt: Register,
    },
    LDRSH {
        rm: Register,
        rn: Register,
        rt: Register,
    },
    LSLImm {
        imm: u32,
        rm: Register,
        rd: Register,
    },
    LSLReg {
        rm: Register,
        rdn: Register,
    },
    LSRImm {
        imm: u32,
        rm: Register,
        rd: Register,
    },
    LSRReg {
        rm: Register,
        rdm: Register,
    },
    MOVImm {
        rd: Register,
        imm: u32,
    },
    MOVRegT1 {
        rm: Register,
        rd: Register,
    },
    MOVRegT2 {
        rm: Register,
        rd: Register,
    },
    MRS {
        rd: Register,
        sysm: SpecialRegister,
    },
    MSRReg {
        rn: Register,
        sysm: SpecialRegister,
    },
    MUL {
        rn: Register,
        rdm: Register,
    },
    MVNReg {
        rm: Register,
        rd: Register,
    },
    NOP,
    ORRReg {
        rm: Register,
        rdn: Register,
    },
    POP {
        reg_list: Vec<Register>,
    },
    PUSH {
        reg_list: Vec<Register>,
    },
    REV {
        rm: Register,
        rd: Register,
    },
    REV16 {
        rm: Register,
        rd: Register,
    },
    REVSH {
        rm: Register,
        rd: Register,
    },
    RORReg {
        rm: Register,
        rdn: Register,
    },
    RSBImm {
        rn: Register,
        rd: Register,
    },
    SBCReg {
        rm: Register,
        rdn: Register,
    },
    SEV,
    STM {
        rn: Register,
        reg_list: Vec<Register>,
    },
    STRImmT1 {
        imm: u32,
        rn: Register,
        rt: Register,
    },
    STRImmT2 {
        rt: Register,
        imm: u32,
    },
    STRReg {
        rm: Register,
        rn: Register,
        rt: Register,
    },
    STRBImm {
        imm: u32,
        rn: Register,
        rt: Register,
    },
    STRBReg {
        rm: Register,
        rn: Register,
        rt: Register,
    },
    STRHImm {
        imm: u32,
        rn: Register,
        rt: Register,
    },
    STRHReg {
        rm: Register,
        rn: Register,
        rt: Register,
    },
    SUBImmT1 {
        imm: u32,
        rn: Register,
        rd: Register,
    },
    SUBImmT2 {
        rdn: Register,
        imm: u32,
    },
    SUBReg {
        rm: Register,
        rn: Register,
        rd: Register,
    },
    SUBImmSP {
        imm: u32,
    },
    SVC {
        imm: u32,
    },
    SXTB {
        rm: Register,
        rd: Register,
    },
    SXTH {
        rm: Register,
        rd: Register,
    },
    TSTReg {
        rm: Register,
        rn: Register,
    },
    UDFT1 {
        imm: u32,
    },
    UDFT2 {
        imm: u32,
    },
    UXTB {
        rm: Register,
        rd: Register,
    },
    UXTH {
        rm: Register,
        rd: Register,
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
