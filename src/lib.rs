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
            let mut instruction_bytes: [u8; 4] = [0; 4];
            instruction_bytes.copy_from_slice(&input[0..4]);
            let instruction_bits = u32::from_le_bytes(instruction_bytes);
            Ok(Instruction {
                width: InstructionWidth::Bit32,
                operation: parse_32bit_operation(instruction_bits)?,
            })
        }
        _ => {
            let mut instruction_bytes: [u8; 2] = [0; 2];
            instruction_bytes.copy_from_slice(&input[0..2]);
            let instruction_bits = u16::from_le_bytes(instruction_bytes);
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
    todo!()
}
