use crate::state::Vault;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
};

// Processes the "deposit" instruction and updates the vault.
pub fn process_deposit(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {

    // Create an iterator over the provided accounts.
    let account_info_iter = &mut accounts.iter();

    // Parse the first 8 bytes of instruction data as a little-endian u64 (the deposit amount).
    let tkn = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    // Retrieve the next account (vault) and verify if it is owned by this program.
    // Macro declare_id! will automatically create an id() function that returns the declared pubkey.
    let vault_info = next_account_info(account_info_iter)?;
    if vault_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    // Mutably borrow the vault account's data to allow modification
    let mut data = vault_info.data.borrow_mut();
    // Interpret the borrowed bytes as a mutable reference to a Vault struct (zero-copy)
    let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);

    vault.deposit(tkn);

    Ok(())
}

pub fn process_withdraw(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let shares = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    let vault_info = next_account_info(account_info_iter)?;
    if vault_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let mut data = vault_info.data.borrow_mut();
    let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);

    vault.withdraw(shares);

    Ok(())
}

pub fn process_reward(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let tkn = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    let vault_info = next_account_info(account_info_iter)?;
    if vault_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let mut data = vault_info.data.borrow_mut();
    let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);

    vault.reward(tkn);

    Ok(())
}

pub fn process_slash(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let tkn = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    let vault_info = next_account_info(account_info_iter)?;
    if vault_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let mut data = vault_info.data.borrow_mut();
    let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);

    vault.slash(tkn);

    Ok(())
}
