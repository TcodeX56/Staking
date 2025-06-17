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

use crate::error::EscrowError;
use crate::state::OwnableAccount;
use crate::state::Users;

pub fn initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let initializer = next_account_info(account_iter)?;
    let ownable_account = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    // let mut data = OwnableAccount::try_from_slice()

    if !initializer.is_signer {
        msg!("Error:Missing Initializer signature");
        return Err(EscrowError::MissingSignature.into());
    }

    let seed_prefix = &[Users::SEED_PREFIX.as_bytes(), initializer.key.as_ref()];

    let (seed_key, bump) = Pubkey::find_program_address(seed_prefix, program_id);

    if ownable_account.owner != program_id {
        let size_a = Users::SIZE + OwnableAccount::SIZE;
        // let size_b = OwnableAccount::SIZE;

        let rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(Users::SIZE + OwnableAccount::SIZE);

        let _ = invoke_signed(
            &system_instruction::create_account(
                initializer.key,
                ownable_account.key,
                rent_lamports,
                size_a as u64,
                program_id
            ),
            &[ownable_account.clone(), initializer.clone(), system_program.clone()],
            &[&[Users::SEED_PREFIX.as_bytes(), initializer.key.as_ref(), &[bump]]]
        );

        msg!("Unpacking state account");

        let mut account_data = Users::try_from_slice(&ownable_account.data.borrow()).map_err(
            |_| ProgramError::InvalidAccountData
        )?;

        let mut data = OwnableAccount::try_from_slice(&ownable_account.data.borrow()).map_err(
            |_| ProgramError::InvalidAccountData
        )?;

        msg!("Borrowed account data");
        msg!("account_data.id,{}", account_data.id);
        msg!("account_data.referral_code,{}", account_data.referral_code);
        msg!(" account_data.referrer,{}", account_data.referrer);

        msg!(" account_data.total_staked_ab,{}", account_data.total_staked_ab);
        msg!(" account_data.data,{}", data.is_initialize);

        account_data.id += 1;
        account_data.referral_code = *initializer.key;
        account_data.referrer = *initializer.key;
        msg!("Serializing account");
        account_data.serialize(&mut &mut ownable_account.data.borrow_mut()[..])?;
        msg!("State account serialized");

        data.is_initialize = true;
        data.owner_account = *initializer.key;

        data.serialize(&mut &mut ownable_account.data.borrow_mut()[..])?;
    }
    let mut data = OwnableAccount::try_from_slice(&ownable_account.data.borrow()).map_err(
        |_| ProgramError::InvalidAccountData
    )?;

    msg!(" account_data.data,{}", data.is_initialize);

    if data.is_initialize {
        return Err(EscrowError::AccountAlreadyInitialized.into());
    }
    Ok(())
}
