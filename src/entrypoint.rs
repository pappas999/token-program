use crate::processor::Processor;
//use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};


//#[cfg(not(feature = "no-entrypoint"))]
//use solana_program::entrypoint;
//#[cfg(not(feature = "no-entrypoint"))]
//entrypoint!(process_instruction);

// Declare and export the program's entrypoint
entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    Processor::process_instruction(program_id, accounts, instruction_data)
}
