use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Users {
    pub id: u8,
    pub referral_code: Pubkey,
    pub referrer: Pubkey,
    pub total_staked_ab: u64,
    pub totalbalance_ab: u64,
    pub balance_ab: u64,
}

impl Users {
    pub const SIZE: usize = 1 + 32 + 32 + 8 + 8 + 8; // = 89 bytes
    pub const SEED_PREFIX: &'static str = "user_authority";
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Liquidity {
    pub amount: u64,
}

impl Liquidity {
    pub const SECURE_SEED: &'static str = "liquidity_authority_with_owner";
    pub const SIZE: usize = 8;
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Amount {
    pub amount: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct WithdrawAmount {
    pub amount: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct OwnableAccount {
    pub is_initialize: bool,
    pub owner_account: Pubkey,
}

impl OwnableAccount {
    pub const SIZE: usize = 50;
}
