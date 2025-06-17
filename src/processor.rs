use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{ account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey };

use crate::instructions::stake::stake_user;
use crate::instructions::withdrew::withdrew_user;
use crate::instructions::initialize::initialize;
// use crate::instructions::initialize
// use crate::state::Users;
use crate::state::Amount;
use crate::state::WithdrawAmount;
use crate::state::OwnableAccount;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum MyInstruction {
    Stake(Amount),
    Withdraw(WithdrawAmount),
    Initializer,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Stake(data) => stake_user(program_id, accounts, data),
        MyInstruction::Withdraw(data) => withdrew_user(program_id, accounts, data),
        MyInstruction::Initializer => initialize(program_id, accounts),
    }
}
