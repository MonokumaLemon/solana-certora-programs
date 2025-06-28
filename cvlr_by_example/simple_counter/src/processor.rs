use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use crate::state::SimpleCounter;


pub fn process_start(
    _program_id: &Pubkey,
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
        _program_id,,
    );

    if expected_pda != *pda_account.key {
        msg!("Provided PDA does not match expected PDA");
        return Err(ProgramError::InvalidArgument);
    }

    // If the PDA account has no data (i.e. not yet initialized),
    // create and initialize it for this user
    if pda_account.data_is_empty() {
        msg!("PDA account is empty. Creating...");

        // Calculate the amount of space (in bytes) for the SimpleCounter struct
        let space = std::mem::size_of::<SimpleCounter>();
        // Calculate the minimum lamports required to make the account rent-exempt
        let rent = Rent::get()?.minimum_balance(space);
        // Create a new PDA-owned account with the required space and rent
        invoke_signed(
            &system_instruction::create_account(
                user.key,
                pda_account.key,
                rent,
                space as u64,
                _program_id,
            ),
            // Signer accounts
            &[user.clone(), pda_account.clone(), system_program.clone()],
            // PDA seeds + bump to authorize signature
            &[&[b"simple_counter", user.key.as_ref(), &[bump]]],
        )?;
        
        let simple_counter = SimpleCounter { ctr: 0 };

        // Serialize and write the counter into the PDA's data space
        simple_counter.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    }
    // Try to read and increment the counter in the PDA account
    let mut  simple_counter = SimpleCounter::try_from_slice(&pda_account.data.borrow())?;
    simple_counter.increment();
    simple_counter.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Greeted {} time(s)", simple_counter.ctr);
    Ok(())
}