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
                let message_length = rest
                    .get(..2)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u16::from_le_bytes)
                    .ok_or(InvalidInstructionData)?;
                let message = (*rest.get(2..).ok_or(InvalidInstructionData)?).to_vec();
                if message.len() > 65535 || message_length != message.len() as u16 {
                    return Err(InvalidInstructionData);
                }
                Self::WriteMessage {
                    sender: *sender,
                    buffer_id: *buffer_id,
                    message_length,
                    message,
                }
            }
            1 => Self::CloseConnection,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
