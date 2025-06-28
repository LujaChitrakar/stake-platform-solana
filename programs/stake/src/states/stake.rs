use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserStake {
    pub user: Pubkey,
    pub stake:Pubkey,
    pub stake_time: i64,
    pub amount_staked: u64,
    pub stake_debt: u64,
    pub pending_reward: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub vault_bump:u8
}

#[account]
#[derive(InitSpace)]
pub struct AdminStake {
    pub admin: Pubkey,
    pub staking_mint: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_rate: u64,
    pub total_staked: u64,
    pub last_update_time: i64,
    pub reward_per_token_stored: u128,
    pub vault_bump:u8
}
