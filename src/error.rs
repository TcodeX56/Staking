
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EscrowError {
    #[error("Invalid Authority")]
    OfferKeyMismatch,

    #[error("Token account provided does not match expected")]
    TokenAccountMismatch,

    #[error("Insufficient user stake")]
    InsufficientUserStake,

    #[error("Insufficient liquidity in vault")]
    InsufficientVaultLiquidity,

    #[error("Unthorized Ownable account")]
    UnauthorizedAccount,

    #[error("Missing signtaure ")]
    MissingSignature,

    #[error("Account already initialized")]
    AccountAlreadyInitialized,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
