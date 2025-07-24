// basic program that doubles the input

use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    pubkey::Pubkey, 
    program_error::ProgramResult,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // logic to double the input number
}