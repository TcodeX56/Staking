use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{ next_account_info, AccountInfo },
    entrypoint::ProgramResult,
    msg,
    program::{ invoke, invoke_signed },
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::state::Amount;
use crate::state::Liquidity;
use crate::state::Users;
use crate::utils::assert_is_associated_token_account;
use spl_associated_token_account::instruction as associated_token_account_instruction;
use spl_token::{ instruction as token_instruction, state::Account as TokenAccount };

pub fn stake_user(program_id: &Pubkey, accounts: &[AccountInfo], data: Amount) -> ProgramResult {
    let account_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
    let mint_account: &AccountInfo<'_> = next_account_info(account_iter)?;
    let pda_account: &AccountInfo<'_> = next_account_info(account_iter)?;
    let pda_account_vault: &AccountInfo<'_> = next_account_info(account_iter)?;
    let from_associated_token_account = next_account_info(account_iter)?;
    let to_associated_token_account: &AccountInfo<'_> = next_account_info(account_iter)?;
    let owner: &AccountInfo<'_> = next_account_info(account_iter)?;
    let recipent: &AccountInfo<'_> = next_account_info(account_iter)?;
    let payer: &AccountInfo<'_> = next_account_info(account_iter)?;
    let system_program: &AccountInfo<'_> = next_account_info(account_iter)?;
    let token_program: &AccountInfo<'_> = next_account_info(account_iter)?;
    let associted_token_program: &AccountInfo<'_> = next_account_info(account_iter)?;

    let seed_prefix: &[&[u8]; 2] = &[Users::SEED_PREFIX.as_bytes(), owner.key.as_ref()];

    let (seed_key, bump) = Pubkey::find_program_address(seed_prefix, program_id);

    msg!("this is pda ,{}", seed_key);

    if pda_account.lamports() == 0 {
        msg!("creating Account");

        let _ = invoke_signed(
            &system_instruction::create_account(
                payer.key,
                pda_account.key,
                Rent::get()?.minimum_balance(Users::SIZE),
                Users::SIZE as u64,
                program_id
            ),
            &[pda_account.clone(), payer.clone(), system_program.clone()],
            &[&[Users::SEED_PREFIX.as_bytes(), owner.key.as_ref(), &[bump]]]
        );
    }

    let liquidity_seed: &[&[u8]; 1] = &[Liquidity::SECURE_SEED.as_bytes()];
    let (liquidity_seed_key, bump_key) = Pubkey::find_program_address(liquidity_seed, program_id);
    msg!("this is pda ,{}", liquidity_seed_key);

    if pda_account_vault.lamports() == 0 {
        msg!("creating PDA account");

        let _ = invoke_signed(
            &system_instruction::create_account(
                payer.key,
                pda_account_vault.key,
                Rent::get()?.minimum_balance(Liquidity::SIZE),
                Liquidity::SIZE as u64,
                program_id
            ),
            &[pda_account_vault.clone(), payer.clone(), system_program.clone()],
            &[&[Liquidity::SECURE_SEED.as_bytes(), &[bump_key]]]
        );
    }

    let _ = assert_is_associated_token_account(
        to_associated_token_account.key,
        pda_account_vault.key,
        mint_account.key
    );

    if to_associated_token_account.lamports() == 0 {
        invoke_signed(
            &associated_token_account_instruction::create_associated_token_account(
                payer.key,
                pda_account_vault.key,
                mint_account.key,
                token_program.key
            ),
            &[
                payer.clone(),
                to_associated_token_account.clone(),
                pda_account_vault.clone(),
                mint_account.clone(),
                system_program.clone(),
                token_program.clone(),
                associted_token_program.clone(),
            ],
            &[&[Liquidity::SECURE_SEED.as_bytes(), &[bump_key]]]
        )?;
    }

    let _ = invoke(
        &token_instruction::transfer(
            token_program.key,
            from_associated_token_account.key,
            to_associated_token_account.key,
            owner.key,
            &[owner.key],
            data.amount
        )?,
        &[
            mint_account.clone(),
            from_associated_token_account.clone(),
            to_associated_token_account.clone(),
            owner.clone(),
            recipent.clone(),
            token_program.clone(),
        ]
    );

    msg!("Unpacking state account");

    let mut account_data: Users = Users::try_from_slice(&pda_account.data.borrow()).map_err(
        |_| ProgramError::InvalidAccountData
    )?;

    msg!("Borrowed account data");
    msg!("account_data.id,{}", account_data.id);
    msg!("account_data.referral_code,{}", account_data.referral_code);
    msg!(" account_data.referrer,{}", account_data.referrer);

    account_data.id += 1;
    account_data.referral_code = *payer.key;
    account_data.referrer = *payer.key;
    account_data.total_staked_ab = account_data.total_staked_ab + data.amount;

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");

    let mut Liquidity_data: Liquidity = Liquidity::try_from_slice(&pda_account_vault.data.borrow()).unwrap();
    msg!("Borrowing account data ");

    Liquidity_data.amount += data.amount;

    Liquidity_data.serialize(&mut &mut pda_account_vault.data.borrow_mut()[..])?;

    Ok(())
}
