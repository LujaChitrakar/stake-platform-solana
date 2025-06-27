use anchor_lang::prelude::*;

#[account]
pub struct UserStake{
    pub user:Pubkey,
    pub stake_time:i64,
    pub amount_staked:u64,
    pub pending_reward:u64,
    pub bump:u8
}

#[account]
pub struct AdminStake{
    pub admin:Pubkey,
    pub staking_mint:Pubkey,
    pub reward_mint:Pubkey,
    pub reward_rate:u64,
    pub total_staked:u64,
    pub last_update_time:i64,
    pub reward_per_token_stored:u128
}