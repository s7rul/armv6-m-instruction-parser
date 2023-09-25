pub mod conditions;
pub mod instructons;
pub mod registers;

use instructons::*;
use registers::*;

pub fn parse(input: &[u8]) -> Result<Instruction, ()> {
    if input.len() < 2 {
        return Err(());
    }

    match (input[0] & 0b11111) >> 2 {
        0b11101 | 0b11110 | 0b11111 => {
            // Check if it is a 32-bit instruction.
            if input.len() < 4 {
                return Err(());
            };
            let mut instruction_bytes1: [u8; 2] = [0; 2];
            let mut instruction_bytes2: [u8; 2] = [0; 2];
            instruction_bytes1.copy_from_slice(&input[0..2]);
            instruction_bytes2.copy_from_slice(&input[2..4]);
            let instruction_bits1 = u16::from_le_bytes(instruction_bytes1);
            let instruction_bits2 = u16::from_le_bytes(instruction_bytes2);
            let instruction_bits: u32 = (instruction_bits1 as u32) << 16 | instruction_bits2 as u32;
            Ok(Instruction {
                width: InstructionWidth::Bit32,
                operation: parse_32bit_operation(instruction_bits)?,
            })
        }
        _ => {
            let mut instruction_bytes: [u8; 2] = [0; 2];
            instruction_bytes.copy_from_slice(&input[0..2]);
            let instruction_bits = u16::from_le_bytes(instruction_bytes);
            println!("instruction bits: {:#018b}", instruction_bits);
            Ok(Instruction {
                width: InstructionWidth::Bit16,
                operation: parse_16bit_operation(instruction_bits)?,
            })
        }
    }
}

fn parse_32bit_operation(input: u32) -> Result<Operation, ()> {
    todo!()
}

fn parse_16bit_operation(input: u16) -> Result<Operation, ()> {
    let opcode = (input >> 10) & 0x3f;
    match opcode {
        0b000000..=0b001111 => {
            todo!() // A5-85
        }
        0b010000 => {
            todo!() // A5-86
        }
        0b010001 => {
            todo!() // A5-86
        }
        0b010010..=0b010011 => {
            todo!() // A6-141
        }
        0b010100..=0b100111 => {
            todo!() // A5-88
        }
        0b101000..=0b101001 => {
            todo!() // A6-115
        }
        0b101010..=0b101011 => {
            todo!() // A6-111
        }
        0b101100..=0b101111 => parse_misc_16_bit(input),
        0b110000..=0b110001 => {
            todo!() // A6-175
        }
        0b110010..=0b110011 => {
            todo!() // A6-137
        }
        0b110100..=0b110111 => {
            todo!() // A5-90
        }
        0b111000..=0b111001 => {
            todo!() // A6-119
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
            let imm = input & 0x7f;
            Ok(Operation::ADDImmSPT2 { imm: imm as u32 })
        }
        0b0000100..=0b0000111 => {
            // SUB SP minus immediate
            // A6-188
            let imm = input & 0x7f;
            Ok(Operation::SUBImmSP { imm: imm as u32 })
        }
        0b0010000..=0b0010001 => {
            // A6-191
            // SXTH
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::SXTH { rm: rm, rd: rd })
        }
        0b0010010..=0b0010011 => {
            // A6-190
            // SXTB
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::SXTB { rm: rm, rd: rd })
        }
        0b0010100..=0b0010101 => {
            // A6-196
            // UXTH
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::UXTH { rm: rm, rd: rd })
        }
        0b0010110..=0b0010111 => {
            // A6-195
            // UXTB
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::UXTB { rm: rm, rd: rd })
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

            Ok(Operation::REV { rm: rm, rd: rd })
        }
        0b1010010..=0b1010011 => {
            // A6-169
            // REV16
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::REV16 { rm: rm, rd: rd })
        }
        0b1010110..=0b1010111 => {
            // A6-170
            // REVSH
            let rm: Register = (((input >> 3) & 0x7) as u8).try_into().unwrap();
            let rd: Register = ((input & 0x7) as u8).try_into().unwrap();

            Ok(Operation::REVSH { rm: rm, rd: rd })
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
