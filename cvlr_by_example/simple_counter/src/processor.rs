use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use crate::state::GreetingAccount;


pub fn process_start(
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {

    // Create an iterator over the provided accounts.
    let account_info_iter = &mut accounts.iter();

    let pda_account = next_account_info(account_info_iter)?; // [0] PDA
    let user = next_account_info(account_info_iter)?;       // [1] User
    let system_program = next_account_info(account_info_iter)?; // [2] SystemProgram

    // Derive the expected PDA address using the seeds and program ID
    let (expected_pda, bump) = Pubkey::find_program_address(
        &[b"simple_counter", user.key.as_ref()],
        program_id,
    );

    if expected_pda != *pda_account.key {
        msg!("Provided PDA does not match expected PDA");
        return Err(ProgramError::InvalidArgument);
    }

    // If the PDA account has no data (i.e. not yet initialized),
    // create and initialize it for this user
    if pda_account.data_is_empty() {
        msg!("PDA account is empty. Creating...");

        // Calculate the amount of space (in bytes) for the GreetingAccount struct
        let space = std::mem::size_of::<GreetingAccount>();
        // Calculate the minimum lamports required to make the account rent-exempt
        let rent = Rent::get()?.minimum_balance(space);
        // Create a new PDA-owned account with the required space and rent
        invoke_signed(
            &system_instruction::create_account(
                user.key,
                pda_account.key,
                rent,
                space as u64,
                program_id,
            ),
            // Signer accounts
            &[user.clone(), pda_account.clone(), system_program.clone()],
            // PDA seeds + bump to authorize signature
            &[&[b"simple_counter", user.key.as_ref(), &[bump]]],
        )?;
        
        let greeting_account = GreetingAccount { counter: 0 };

        // Serialize and write the counter into the PDA's data space
        greeting_account.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    }


    Ok(())
}