use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    info,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::ZoolanaInstruction;

pub struct Processor {}

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let ix = ZoolanaInstruction::unpack(instruction_data)?;

        match ix {
            ZoolanaInstruction::WriteMessage {
                sender,
                signal_id,
                message_parts,
                message_part_id,
                message_length,
                message,
            } => {
                info!("Processing write message ix");
                Self::process_write_message(
                    accounts,
                    sender,
                    signal_id,
                    message_parts,
                    message_part_id,
                    message_length,
                    message,
                )
            }
            _ => Ok(()),
        }
    }

    fn process_write_message(
        accounts: &[AccountInfo],
        sender: u8,
        signal_id: u8,
        message_parts: u8,
        message_part_id: u8,
        message_length: u16,
        message: Vec<u8>,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let connection_account_info = next_account_info(account_info_iter)?;

        if !connection_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let account_data = &mut connection_account_info.data.borrow_mut();

        if account_data.len() != 20000 {
            return Err(ProgramError::InvalidAccountData);
        }

        let mut i = 0;
        loop {
            if account_data[i] == 0 {
                account_data[i] = sender;
                i += 1;
                account_data[i] = signal_id;
                i += 1;
                account_data[i] = message_parts;
                i += 1;
                account_data[i] = message_part_id;
                let message_length_bytes = message_length.to_le_bytes();
                i += 1;
                account_data[i] = message_length_bytes[0];
                i += 1;
                account_data[i] = message_length_bytes[1];
                i += 1;
                account_data[i..i + message_length as usize].copy_from_slice(&message);
                break;
            } else {
                let current_message_length =
                    u16::from_le_bytes([account_data[i + 4], account_data[i + 5]]);

                i += (current_message_length + 6) as usize;
            }
        }
        Ok(())
    }
}
