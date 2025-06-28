use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Staking Mint")]
    InvalidStakingMint,
    #[msg("Invalid Stake")]
    InvalidStake,
    #[msg("Invalid User Ata")]
    InvalidUserAta,
    #[msg("Stake time not completed yet.")]
    StakeTimeNotCompleted,
    #[msg("The amount to unstake is greater than staked amount.")]
    AmountGreaterThanStakedAmount,
}
