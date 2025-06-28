use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, pubkey::Pubkey,
};

mod processor;
mod state;

// Include formal verification module only if certora feature is enabled.
#[cfg(feature = "certora")]
mod certora;

#[cfg(not(feature = "certora"))]
use solana_program::msg;
// If certora feature is enabled, msg should be substituted with `clog!`.
#[cfg(feature = "certora")]
use cvlr::clog as msg;

//Global counter: declare_id!("5Q7SWnUeRtPskf2H51JQVvgDFSKMSbzHMLD9hT2M22zn");
declare_id!("FNuYKdEuhwGFp23UuQk9P4Hh9VktotebvrR6Xnmd2m4S");

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_start(_program_id, accounts)?;
    Ok(())
}