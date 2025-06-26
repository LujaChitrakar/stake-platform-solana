use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::Metadata,
    token::{Mint, Token},
};

#[derive(Accounts)]
#[instruction(_token_decimals:u8)]
pub struct CreateTokenMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    ///CHECK:Validate address by deriving pda
    #[account(
        mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(),mint_account.key().as_ref()],
        bump,
        seeds::program=token_metadata_program.key()
    )]
    pub metadata_account: UncheckedAccount<'info>,

    #[account(
        init,
        payer=payer,
        mint::decimals=_token_decimals,
        mint::authority=payer.key(),
        mint::freeze_authority=payer.key()
    )]
    pub mint_account: Account<'info, Mint>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
