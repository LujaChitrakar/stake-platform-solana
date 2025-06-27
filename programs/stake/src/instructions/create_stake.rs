use crate::AdminStake;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateStake<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        space=AdminStake::INIT_SPACE,
        seeds=[b"stake",admin.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, AdminStake>,

    #[account(
        init,
        payer=admin,
        seeds=[b"vault",stake.key().as_ref()],
        bump,
        token::mint=staking_mint,
        token::authority=vault_authority
    )]
    pub vault: Account<'info, TokenAccount>,

    ///CHECK: This account is only for authority. It doesnt read or write.
    #[account(seeds=[b"vault_authority",vault.key().as_ref()],bump)]
    pub vault_authority: UncheckedAccount<'info>,

    pub staking_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}
