use anchor_lang::prelude::*;

#[event]
pub struct StakePlaced {
    pub user: Pubkey,
    pub stake_amount: u64,
    pub stake_time: i64,
}
