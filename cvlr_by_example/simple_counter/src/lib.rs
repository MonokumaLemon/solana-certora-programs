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

declare_id!("2gGKhQqfdHxpK5wo66KJ8aY4XvjxjUattmzoA1u85Vyk");

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_start(accounts)
    Ok(())
}