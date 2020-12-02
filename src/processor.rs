use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::ZoolanaInstruction;

pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let ix = ZoolanaInstruction::unpack(instruction_data);

        let account_info_iter = &mut accounts.iter();
        let connection_account_info = next_account_info(account_info_iter)?;

        if !connection_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(())
    }
}
