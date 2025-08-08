use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount},
};

use crate::{AdminStake, UserStake};

#[derive(Accounts)]
pub struct StakeToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[b"stake",stake.admin.key().as_ref()],
        bump)]
    pub stake: Account<'info, AdminStake>,

    #[account(
        init_if_needed,
        payer=user,
        seeds=[b"user_stake",user.key().as_ref(),stake.key().as_ref()],
        space=8+UserStake::INIT_SPACE,
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    #[account(
        mut,
        seeds=[b"vault",stake.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=stake.staking_mint,associated_token::authority=user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
