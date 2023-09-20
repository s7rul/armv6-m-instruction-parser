pub mod instructons;
pub mod registers;

use instructons::*;
use registers::*;

pub fn parse(input: &[u8]) -> Result<Instruction, ()> {
    if input.len() < 2 {
        return Err(());
    }

    match (input[0] & 0b11111) >> 10 {
        0b11101 | 0b11110 | 0b11111 => {
            // Check if it is a 32-bit instruction.
            Ok(Instruction::Bit32(parse_32bit_instruction(input)?))
        }
        _ => Ok(Instruction::Bit16(parse_16bit_instruction(input)?)),
    }
}

fn parse_32bit_instruction(input: &[u8]) -> Result<Instruction32Bit, ()> {
    todo!()
}

fn parse_16bit_instruction(input: &[u8]) -> Result<Instruction16Bit, ()> {
    todo!()
}
