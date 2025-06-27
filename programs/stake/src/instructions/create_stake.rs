use anchor_lang::prelude::*;

use crate::Stake;


#[derive(Accounts)]
pub struct CreateStake<'info>{
    #[account(mut)]
    pub owner:Signer<'info>,

    #[account(
        init,
        payer=owner,
        seeds=[b"stake",owner.key()],
        bump
    )]
    pub stake:Account<'info,Stake>,
}