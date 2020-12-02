use solana_program::program_error::{ProgramError, ProgramError::InvalidInstructionData};

use std::convert::TryInto;

pub enum ZoolanaInstruction {
    WriteMessage {
        sender: u8,
        buffer_id: u8,
        message_length: u16,
        message: Vec<u8>,
    },
    CloseConnection,
}

impl ZoolanaInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (sender, rest) = rest.split_first().ok_or(InvalidInstructionData)?;
                let (buffer_id, rest) = rest.split_first().ok_or(InvalidInstructionData)?;
                Self::WriteMessage {
                    sender: *sender,
                    buffer_id: *buffer_id,
                    message_length: rest
                        .get(..2)
                        .and_then(|slice| slice.try_into().ok())
                        .map(u16::from_le_bytes)
                        .ok_or(InvalidInstructionData)?,
                    message: (*rest.get(2..).ok_or(InvalidInstructionData)?).to_vec(),
                }
            }
            1 => Self::CloseConnection,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
