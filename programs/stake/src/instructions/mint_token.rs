use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

#[derive(Accounts)]
pub struct MintToken<'info>{
    #[account(mut)]
    pub mint_authority:Signer<'info>,

    #[account(mut)]
    pub mint_account:Account<'info,Mint>,

    pub recipient:SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer=mint_authority,
        associated_token::mint=mint_account,
        associated_token::authority=recipient,
    )]
    pub associated_token_account:Account<'info,TokenAccount>,

    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>
}   