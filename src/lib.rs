//! Library to parse ARMv6-M thumb instructions.
//!
//! Provides a enum with all instructions, register types and a function to parse binary representation into the enum with proper arguments.
//!
//! # Example
//! ```
//! # use armv6_m_instruction_parser::parse;
//! # fn main() {
//! #   let program_memory = [0xb0, 0xb5, 0xaf, 0x02];
//!     match parse(&program_memory[0..4]) {
//!         Ok(instruction) => println!("Instruction: {:?}", instruction),
//!         Err(_) => println!("Not a valid instruction.")
//!     }
//! # }
//! ```

pub mod conditions;
pub mod instructons;
pub mod registers;

use conditions::Condition;
use instructons::*;
use registers::*;
use tracing::debug;

/// This function parses a input byte slice into one instruction.
/// Returns Err(()) if instruction is invalid.
pub fn parse(input: &[u8]) -> Result<Instruction, ()> {
    if input.len() < 2 {
        return Err(());
    }
    let mut instruction_bytes1: [u8; 2] = [0; 2];
    instruction_bytes1.copy_from_slice(&input[0..2]);
    let instruction_bits1 = u16::from_le_bytes(instruction_bytes1);

    match (instruction_bits1 >> 11) & 0x1f {
        0b11101 | 0b11110 | 0b11111 => {
            // Check if it is a 32-bit instruction.
            if input.len() < 4 {
                return Err(());
            };
            let mut instruction_bytes2: [u8; 2] = [0; 2];
            instruction_bytes2.copy_from_slice(&input[2..4]);
            let instruction_bits2 = u16::from_le_bytes(instruction_bytes2);
            let instruction_bits: u32 = (instruction_bits1 as u32) << 16 | instruction_bits2 as u32;
            debug!("instruction bits: {:#034b}",instruction_bits);
            Ok(Instruction {
                width: InstructionWidth::Bit32,
                operation: parse_32bit_operation(instruction_bits)?,
            })
        }
        _ => {
            debug!("instruction bits: {:#018b}",instruction_bits1);
            Ok(Instruction {
                width: InstructionWidth::Bit16,
                operation: parse_16bit_operation(instruction_bits1)?,
            })
        }
    }
}

fn parse_32bit_operation(input: u32) -> Result<Operation, ()> {
    let op1 = (input >> 27) & 0x3;
    let op = (input >> 15) & 0x1;

    match (op1, op) {
        (0b10, 0b1) => {
            // brach and misc control
            parse_branch_misc_ctrl(input)
        }
        (_, _) => Err(()),
    }
}

fn parse_branch_misc_ctrl(input: u32) -> Result<Operation, ()> {
    let op1 = (input >> 20) & 0x7f;
    let op2 = (input >> 12) & 0x7;

    match (op2, op1) {
        (0b000 | 0b010, 0b0111000..=0b0111001) => {
            // MSR
            let rn = (((input >> 16) & 0xf) as u8).try_into().unwrap();
            let sysm = ((input & 0xff) as u8).try_into()?; // can fail
            Ok(Operation::MSRReg { n: rn, sysm: sysm })
        }
        (0b000 | 0b010, 0b0111011) => {
            // misc control
            parse_misc_ctrl(input)
        }
        (0b000 | 0b010, 0b0111110..=0b0111111) => {
            // MRS
            let rd = (((input >> 8) & 0xf) as u8).try_into().unwrap();
            let sysm = ((input & 0xff) as u8).try_into()?;
            Ok(Operation::MRS { d: rd, sysm: sysm })
        }
        (0b111, 0b1111111) => {
            // Permanently Undefined
            Err(())
        }
        (0b101 | 0b111, _) => {
            // BL
            let s = (input >> 26) & 0x1;
            let j1 = (input >> 13) & 0x1;
            let j2 = (input >> 11) & 0x1;
            let i1 = !(j1 ^ s) & 0x1;
            let i2 = !(j2 ^ s) & 0x1;
            let imm10 = (input >> 16) & 0x3ff;
            let imm11 = input & 0x7ff;
            let imm = ((s << 24) | (i1 << 23) | (i2 << 22) | (imm10 << 12) | (imm11 << 1))
                .sign_extend(25);

            Ok(Operation::BL { imm: imm })
        }
        _ => Err(()),
    }
}

fn parse_misc_ctrl(input: u32) -> Result<Operation, ()> {
    let op = (input >> 4) & 0xf;

    match op {
        0b0100 => {
            let option = input & 0xf;
            Ok(Operation::DSB {
                option: option as u8,
            })
        }
        0b0101 => {
            let option = input & 0xf;
            Ok(Operation::DMB {
                option: option as u8,
            })
        }
        0b0110 => {
            let option = input & 0xf;
            Ok(Operation::ISB {
                option: option as u8,
            })
        }
        _ => Err(()),
    }
}

fn parse_16bit_operation(input: u16) -> Result<Operation, ()> {
    let opcode = (input >> 10) & 0x3f;
    match opcode {
        0b000000..=0b001111 => {
            parse_arith_instructions(input) // A5-85
        }
        0b010000 => {
            parse_data_processing_instruction(input) // A5-86
        }
        0b010001 => {
            parse_special_data_branch_exchange_instruction(input) // A5-86
        }
        0b010010..=0b010011 => {
            // A6-141
            // LDR literal
            let rt: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
            let imm = ((input & 0xff) << 2) as u32;
            Ok(Operation::LDRLiteral { t: rt, imm: imm })
        }
        0b010100..=0b100111 => {
            // A5-88
            parse_load_store_instruction(input)
        }
        0b101000..=0b101001 => {
            // A6-115
            let rd: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
            let imm = ((input & 0xff) << 2) as u32;
            Ok(Operation::ADR { d: rd, imm: imm })
        }
        0b101010..=0b101011 => {
            // A6-111
            let rd: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
            let imm = ((input & 0xff) << 2) as u32;
            Ok(Operation::ADDImmSP { d: rd, imm: imm })
        }
        0b101100..=0b101111 => {
            parse_misc_16_bit(input) // A5-86
        }
        0b110000..=0b110001 => {
            // A6-175
            let rn: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
            let reg_list_bits = input & 0xff;
            let reg_list = register_list_from_bit_array(reg_list_bits);
            Ok(Operation::STM {
                n: rn,
                reg_list: reg_list,
            })
        }
        0b110010..=0b110011 => {
            // A6-137
            let rn: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
            let reg_list_bits = input & 0xff;
            let reg_list = register_list_from_bit_array(reg_list_bits);
            Ok(Operation::LDM {
                n: rn,
                reg_list: reg_list,
            })
        }
        0b110100..=0b110111 => {
            // A5-90
            parse_conditional_branch(input)
        }
        0b111000..=0b111001 => {
            // A6-119
            // Unconditional branch
            let imm = ((input & 0x7ff) << 1).sign_extend(12);
            Ok(Operation::B {
                cond: Condition::None,
                imm: imm,
            })
        }
        _ => Err(()),
    }
}

fn parse_conditional_branch(input: u16) -> Result<Operation, ()> {
    let opcode = (input >> 8) & 0xf;

    match opcode {
        0b1110 => Err(()), // Permanently undefined
        0b1111 => {
            // SVC
            let imm = (input & 0xff) as u32;
            Ok(Operation::SVC { imm: imm })
        }
        _ => {
            // B
            let imm = ((input & 0xff) << 1).sign_extend(9);
            let cond: Condition = (((input >> 8) & 0xf) as u8).try_into()?;
            Ok(Operation::B {
                cond: cond,
                imm: imm,
            })
        }
    }
}

fn parse_load_store_instruction(input: u16) -> Result<Operation, ()> {
    let op_a = (input >> 12) & 0xf;
    let op_b = (input >> 9) & 0x7;

    match (op_a, op_b) {
        (0b0101, op_b) => match op_b {
            0b000 => {
                // STR reg
                let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                Ok(Operation::STRReg {
                    m: rm,
                    n: rn,
                    t: rt,
                })
            }
            0b001 => {
                // STRH reg
                let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                Ok(Operation::STRHReg {
                    m: rm,
                    n: rn,
                    t: rt,
                })
            }
            0b010 => {
                // STRB reg
                let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                Ok(Operation::STRBReg {
                    m: rm,
                    n: rn,
                    t: rt,
                })
            }
            0b011 => {
                // LDRSB reg
                let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                Ok(Operation::LDRSBReg {
                    m: rm,
                    n: rn,
                    t: rt,
                })
            }
            0b100 => {
                // LDR reg
                let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                Ok(Operation::LDRReg {
                    m: rm,
                    n: rn,
                    t: rt,
                })
            }
            0b101 => {
                // LDRH reg
                let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                Ok(Operation::LDRHReg {
                    m: rm,
                    n: rn,
                    t: rt,
                })
            }
            0b110 => {
                // LDRB reg
                let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                Ok(Operation::LDRBReg {
                    m: rm,
                    n: rn,
                    t: rt,
                })
            }
            0b111 => {
                // LDRSH reg
                let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                Ok(Operation::LDRSH {
                    m: rm,
                    n: rn,
                    t: rt,
                })
            }
            _ => Err(()),
        },
        (0b0110, op_b) => match op_b {
            0b000..=0b011 => {
                // STR
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                let imm = ((input & 0x7c0) >> 4) as u32;
                Ok(Operation::STRImm {
                    imm: imm,
                    n: rn,
                    t: rt,
                })
            }
            0b100..=0b111 => {
                // LDR
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                let imm = ((input & 0x7c0) >> 4) as u32;
                Ok(Operation::LDRImm {
                    imm: imm,
                    n: rn,
                    t: rt,
                })
            }
            _ => Err(()),
        },
        (0b0111, op_b) => match op_b {
            0b000..=0b011 => {
                // STRB
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                let imm = ((input & 0x7c0) >> 6) as u32;
                Ok(Operation::STRBImm {
                    imm: imm,
                    n: rn,
                    t: rt,
                })
            }
            0b100..=0b111 => {
                // LDRB
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                let imm = ((input & 0x7c0) >> 6) as u32;
                Ok(Operation::LDRBImm {
                    imm: imm,
                    n: rn,
                    t: rt,
                })
            }
            _ => Err(()),
        },
        (0b1000, op_b) => match op_b {
            0b000..=0b011 => {
                // STRH
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                let imm = ((input & 0x7c0) >> 5) as u32;
                Ok(Operation::STRHImm {
                    imm: imm,
                    n: rn,
                    t: rt,
                })
            }
            0b100..=0b111 => {
                // LDRH
                let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
                let rt: Register = ((input & 0x7) as u8).try_into().unwrap();
                let imm = ((input & 0x7c0) >> 5) as u32;
                Ok(Operation::LDRHImm {
                    imm: imm,
                    n: rn,
                    t: rt,
                })
            }
            _ => Err(()),
        },
        (0b1001, op_b) => match op_b {
            0b000..=0b011 => {
                // STR T2
                let rt: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
                let imm = ((input & 0xff) << 2) as u32;
                Ok(Operation::STRImm {
                    n: Register::SP,
                    t: rt,
                    imm: imm,
                })
            }
            0b100..=0b111 => {
                // LDR T2
                let rt: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
                let imm = ((input & 0xff) << 2) as u32;
                Ok(Operation::LDRImm {
                    n: Register::SP,
                    t: rt,
                    imm: imm,
                })
            }
            _ => Err(()),
        },
        (_, _) => Err(()),
    }
}

fn parse_special_data_branch_exchange_instruction(input: u16) -> Result<Operation, ()> {
    let opcode = (input >> 6) & 0xf;
    match opcode {
        0b0000..=0b0011 => {
            // 01000100xx
            let rm: Register = (((input >> 3) & 0xf) as u8).try_into().unwrap();
            let rdn: Register = (((input & 0x7) | ((input >> 4) & 0b1000)) as u8)
                .try_into()
                .unwrap();
            if rdn == Register::SP || rm == Register::SP {
                if rm == Register::SP {
                    // T1
                    Ok(Operation::ADDRegSP { d: rdn, m: rdn })
                } else {
                    // T2
                    Ok(Operation::ADDRegSP {
                        d: Register::SP,
                        m: rm,
                    })
                }
            } else {
                // ADD reg T2
                Ok(Operation::ADDReg {
                    m: rm,
                    n: rdn,
                    d: rdn,
                })
            }
        }
        0b0100 => Err(()), // Unpredictable
        0b0101 | 0b0110..=0b0111 => {
            let rm: Register = (((input >> 3) & 0xf) as u8).try_into().unwrap();
            let rn: Register = (((input & 0x7) | ((input >> 4) & 0b1000)) as u8)
                .try_into()
                .unwrap();
            Ok(Operation::CMPReg { m: rm, n: rn })
        }
        0b1000..=0b1011 => {
            let rm: Register = (((input >> 3) & 0xf) as u8).try_into().unwrap();
            let rd: Register = (((input & 0x7) | ((input >> 4) & 0b1000)) as u8)
                .try_into()
                .unwrap();
            Ok(Operation::MOVReg {
                set_flags: false,
                m: rm,
                d: rd,
            })
        }
        0b1100..=0b1101 => {
            let rm: Register = (((input >> 3) & 0xf) as u8).try_into().unwrap();
            Ok(Operation::BX { m: rm })
        }
        0b1110..=0b1111 => {
            let rm: Register = (((input >> 3) & 0xf) as u8).try_into().unwrap();
            Ok(Operation::BLXReg { m: rm })
        }
        _ => Err(()),
    }
}

fn parse_data_processing_instruction(input: u16) -> Result<Operation, ()> {
    let opcode = (input >> 6) & 0xf;
    match opcode {
        0b0000 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::ANDReg { m: rm, dn: rdn })
        }
        0b0001 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::EORReg { m: rm, dn: rdn })
        }
        0b0010 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::LSLReg { m: rm, dn: rdn })
        }
        0b0011 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::LSRReg { m: rm, dn: rdn })
        }
        0b0100 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::ASRReg { m: rm, dn: rdn })
        }
        0b0101 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::ADCReg {
                m: rm,
                n: rdn,
                d: rdn,
            })
        }
        0b0110 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::SBCReg { m: rm, dn: rdn })
        }
        0b0111 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::RORReg { m: rm, dn: rdn })
        }
        0b1000 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::TSTReg { m: rm, n: rn })
        }
        0b1001 => {
            let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::RSBImm { n: rn, d: rd })
        }
        0b1010 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::CMPReg { m: rm, n: rn })
        }
        0b1011 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::CMNReg { m: rm, n: rn })
        }
        0b1100 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::ORRReg { m: rm, dn: rdn })
        }
        0b1101 => {
            let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdm: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::MUL { n: rn, dm: rdm })
        }
        0b1110 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rdn: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::BICReg { m: rm, dn: rdn })
        }
        0b1111 => {
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::MVNReg { m: rm, d: rd })
        }
        _ => Err(()),
    }
}

fn parse_arith_instructions(input: u16) -> Result<Operation, ()> {
    // A5-85
    let opcode = (input >> 9) & 0x1f;
    match opcode {
        0b00000..=0b00011 => {
            //LSL
            let imm = (input >> 6) & 0x1f;
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();
            if imm > 0 {
                Ok(Operation::LSLImm {
                    imm: imm as u32,
                    m: rm,
                    d: rd,
                })
            } else {
                Ok(Operation::MOVReg {
                    set_flags: true,
                    m: rm,
                    d: rd,
                })
            }
        }
        0b00100..=0b00111 => {
            //LSR
            let imm = (input >> 6) & 0x1f;
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::LSRImm {
                imm: imm as u32,
                m: rm,
                d: rd,
            })
        }
        0b01000..=0b01011 => {
            //ASR
            let imm = (input >> 6) & 0x1f;
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::ASRImm {
                imm: imm as u32,
                m: rm,
                d: rd,
            })
        }
        0b01100 => {
            // ADD reg
            let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
            let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::ADDReg {
                m: rm,
                n: rn,
                d: rd,
            })
        }
        0b01101 => {
            // SUB reg
            let rm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
            let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::SUBReg {
                m: rm,
                n: rn,
                d: rd,
            })
        }
        0b01110 => {
            // ADD 3bit imm
            let imm = (input >> 6) & 0x7;
            let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::ADDImm {
                imm: imm as u32,
                n: rn,
                d: rd,
            })
        }
        0b01111 => {
            // SUB 3bit imm
            let imm: Register = (((input >> 6) & 0x7) as u8).try_into().unwrap();
            let rn: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();
            Ok(Operation::SUBImm {
                imm: imm as u32,
                n: rn,
                d: rd,
            })
        }
        0b10000..=0b10011 => {
            // MOV imm
            let imm = input & 0xff;
            let rd: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
            Ok(Operation::MOVImm {
                d: rd,
                imm: imm as u32,
            })
        }
        0b10100..=0b10111 => {
            // CMP imm
            let imm = input & 0xff;
            let rn: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
            Ok(Operation::CMPImm {
                n: rn,
                imm: imm as u32,
            })
        }
        0b11000..=0b11011 => {
            // ADD 8bit imm
            let imm = input & 0xff;
            let rdn: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
            Ok(Operation::ADDImm {
                imm: imm as u32,
                n: rdn,
                d: rdn,
            })
        }
        0b11100..=0b11111 => {
            // SUB 8bit imm
            let imm = input & 0xff;
            let rdn: Register = (((input >> 8) & 0x7) as u8).try_into().unwrap();
            Ok(Operation::SUBImm {
                n: rdn,
                d: rdn,
                imm: imm as u32,
            })
        }
        _ => Err(()),
    }
}

fn parse_misc_16_bit(input: u16) -> Result<Operation, ()> {
    let opcode = (input >> 5) & 0x7f;
    match opcode {
        0b0000000..=0b0000011 => {
            // ADD SP plus immediate
            // A6-111
            let imm = (input & 0x7f) << 2;
            Ok(Operation::ADDImmSP {
                d: Register::SP,
                imm: imm as u32,
            })
        }
        0b0000100..=0b0000111 => {
            // SUB SP minus immediate
            // A6-188
            let imm = (input & 0x7f) << 2;
            Ok(Operation::SUBImmSP { imm: imm as u32 })
        }
        0b0010000..=0b0010001 => {
            // A6-191
            // SXTH
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::SXTH { m: rm, d: rd })
        }
        0b0010010..=0b0010011 => {
            // A6-190
            // SXTB
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::SXTB { m: rm, d: rd })
        }
        0b0010100..=0b0010101 => {
            // A6-196
            // UXTH
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::UXTH { m: rm, d: rd })
        }
        0b0010110..=0b0010111 => {
            // A6-195
            // UXTB
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::UXTB { m: rm, d: rd })
        }
        0b0100000..=0b0101111 => {
            // PUSH
            // A6-167
            let reg_list_bits = (((input >> 8) & 0b1) << 14) | (input & 0xff);
            let reg_list = register_list_from_bit_array(reg_list_bits);
            Ok(Operation::PUSH { reg_list: reg_list })
        }
        0b0110011 => {
            // B4-306
            // CPS
            let im = ((input >> 4) & 0b1) == 1;
            Ok(Operation::CPS { im: im })
        }
        0b1010000..=0b1010001 => {
            // A6-168
            // REV
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::REV { m: rm, d: rd })
        }
        0b1010010..=0b1010011 => {
            // A6-169
            // REV16
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::REV16 { m: rm, d: rd })
        }
        0b1010110..=0b1010111 => {
            // A6-170
            // REVSH
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::REVSH { m: rm, d: rd })
        }
        0b1100000..=0b1101111 => {
            // A6-165
            // POP
            let reg_list_bits = (((input >> 8) & 0b1) << 15) | (input & 0xff);
            let reg_list = register_list_from_bit_array(reg_list_bits);
            Ok(Operation::POP { reg_list: reg_list })
        }
        0b1110000..=0b1110111 => {
            // A6-122
            // BKPT
            let imm = input & 0xff;
            Ok(Operation::BKPT { imm: imm as u32 })
        }
        0b1111000..=0b1111111 => {
            // A5-90
            // Hint instruction
            parse_hint_instruction(input)
        }
        _ => Err(()),
    }
}

fn parse_hint_instruction(input: u16) -> Result<Operation, ()> {
    // A5-90
    let op_a = (input >> 4) & 0xf;
    let op_b = input & 0xf;

    if op_b > 0 {
        return Err(());
    }

    match op_a {
        0b0000 => Ok(Operation::NOP),
        0b0001 => Ok(Operation::YIELD),
        0b0010 => Ok(Operation::WFE),
        0b0011 => Ok(Operation::WFE),
        0b0100 => Ok(Operation::SEV),
        _ => Err(()),
    }
}

trait SignExtend {
    fn sign_extend(&self, valid_bits: usize) -> u32;
}

impl SignExtend for u16 {
    fn sign_extend(&self, valid_bits: usize) -> u32 {
        let shift = 16 - valid_bits;
        ((((self << shift) as i16) >> shift) as i32) as u32
    }
}

impl SignExtend for u32 {
    fn sign_extend(&self, valid_bits: usize) -> u32 {
        let shift = 32 - valid_bits;
        (((self << shift) as i32) >> shift) as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sign_extend_u16() {
        assert_eq!(0xffffffff, 0x1u16.sign_extend(1));
        assert_eq!(0x1, 0x1u16.sign_extend(2));
        assert_eq!(0xfffffff9, 0x9u16.sign_extend(4));
        assert_eq!(0x00000009, 0x9u16.sign_extend(5));
    }

    #[test]
    fn sign_extend_u32() {
        assert_eq!(0xffffffff, 0x1u32.sign_extend(1));
        assert_eq!(0x1, 0x1u32.sign_extend(2));
        assert_eq!(0xfffffff9, 0x9u32.sign_extend(4));
        assert_eq!(0x00000009, 0x9u32.sign_extend(5));
    }
}
