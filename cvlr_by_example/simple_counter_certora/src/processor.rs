use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
};
use crate::state::SimpleCounter;

pub fn process_start(
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {

    // Create an iterator over the provided accounts.
    let account_info_iter = &mut accounts.iter();

    let account_info = next_account_info(account_info_iter)?;
    let mut data = account_info.data.borrow_mut();
    // Interpret the borrowed bytes as a mutable reference to a Vault struct (zero-copy)
    let simplecounter: &mut SimpleCounter = bytemuck::from_bytes_mut(&mut data[..]);
    simplecounter.increment();
    Ok(())
}