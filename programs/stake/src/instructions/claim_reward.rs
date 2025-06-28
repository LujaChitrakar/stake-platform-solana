pub use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount}};

use crate::{AdminStake, UserStake};

#[derive(Accounts)]
pub struct ClaimReward<'info>{
     #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[b"stake",stake.admin.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, AdminStake>,

    #[account(
        mut,
        seeds=[b"user_stake",user.key().as_ref(),stake.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    #[account(
        mut,
        seeds=[b"vault",stake.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    ///CHECK: This account is only for authority. It doesnt read or write.
    #[account(seeds=[b"vault_authority",vault.key().as_ref()],bump)]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint=stake.staking_mint,
        associated_token::authority=user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}