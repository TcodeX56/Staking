use borsh::{ BorshDeserialize, BorshSerialize };

use solana_program::{
    account_info::{ next_account_info, AccountInfo },
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::EscrowError;
use crate::state::Liquidity;
use crate::state::Users;
use crate::state::WithdrawAmount;

use spl_token::instruction as token_instruction;

pub fn withdrew_user(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: WithdrawAmount
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let _mint_account = next_account_info(account_iter)?;
    let pda_account = next_account_info(account_iter)?;
    let pda_account_vault = next_account_info(account_iter)?;
    let from_associated_token_account = next_account_info(account_iter)?;
    let to_associated_token_account = next_account_info(account_iter)?;
    let owner = next_account_info(account_iter)?;
    let _recipent = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let _system_program = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let _associted_token_program = next_account_info(account_iter)?;

    let seed_prefix = &[Users::SEED_PREFIX.as_bytes(), owner.key.as_ref()];

    let (seed_key, bump) = Pubkey::find_program_address(seed_prefix, program_id);

    let liquidity_seed = &[Liquidity::SECURE_SEED.as_bytes()];
    let (_liquidity_seed_key, bump_key) = Pubkey::find_program_address(liquidity_seed, program_id);

    let mut account_data = Users::try_from_slice(&pda_account.data.borrow()).map_err(
        |_| ProgramError::InvalidAccountData
    )?;

    msg!("Deserialized user account, stake value: {}", account_data.total_staked_ab);

    msg!("User stake: {}", account_data.total_staked_ab);
    msg!("Withdraw request: {}", data.amount);

    if account_data.total_staked_ab < data.amount {
        msg!("Error: Insufficient user stake: {} < {}", account_data.total_staked_ab, data.amount);
        return Err(EscrowError::InsufficientUserStake.into());
    }

    let mut Liquidity_data = Liquidity::try_from_slice(&pda_account_vault.data.borrow()).unwrap();
    msg!("Borrowing account data ");

    if Liquidity_data.amount < data.amount {
        return Err(EscrowError::InsufficientVaultLiquidity.into());
    }

    msg!("Borrowed account data");
    msg!("account_data.id,{}", account_data.id);
    msg!("account_data.referral_code,{}", account_data.referral_code);
    msg!(" account_data.referrer,{}", account_data.referrer);
    msg!(" account_data.total_staked_ab,{}", account_data.total_staked_ab);
    msg!(" account_data.Liquidity_data,{}", Liquidity_data.amount);

    invoke_signed(
        &token_instruction::transfer(
            token_program.key,
            from_associated_token_account.key, // Source: PDA vault token account
            to_associated_token_account.key, // Destination: User's token account
            pda_account_vault.key, // Authority: must be PDA
            &[pda_account_vault.key, payer.key], // No multisig: PDA will sign via invoke_signed
            data.amount
        )?,
        &[
            from_associated_token_account.clone(),
            to_associated_token_account.clone(),
            pda_account_vault.clone(), // PDA authority
            payer.clone(),
            token_program.clone(),
        ],
        &[&[Liquidity::SECURE_SEED.as_bytes(), &[bump_key]]] // PDA seed & bump
    )?;

    account_data.total_staked_ab -= data.amount;
    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");

    Liquidity_data.amount -= data.amount;
    Liquidity_data.serialize(&mut &mut pda_account_vault.data.borrow_mut()[..])?;
    Ok(())
}
