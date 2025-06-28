use anchor_lang::prelude::*;
pub mod error;
pub mod event;
pub mod instructions;
pub mod states;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
    },
    token::{mint_to, MintTo},
};
pub use error::ErrorCode;
pub use event::*;
pub use instructions::*;
pub use states::*;

declare_id!("3Yotwu7h86XRhWEbVnbAUJQGxmBEypEwN5ZoLQBXrT8G");

#[program]
pub mod stake {
    use anchor_spl::token::{transfer, Transfer};

    use super::*;

    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>,
        _token_decimals: u8,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        msg!("Creating a metadata account");
        msg!(
            "Metadata account address:{}",
            &ctx.accounts.metadata_account.key()
        );

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.payer.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );

        let data = DataV2 {
            name: token_name,
            symbol: token_symbol,
            uri: token_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_ctx, data, false, true, None)?;

        msg!("Token created successfully");
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        msg!("Minting token to the associated token account..");
        msg!("Mint {}", &ctx.accounts.mint_account.key());
        msg!(
            "Token address: {}",
            &ctx.accounts.associated_token_account.key()
        );

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        );
        let amount_to_mint = amount * 10u64.pow(ctx.accounts.mint_account.decimals as u32);

        mint_to(cpi_ctx, amount_to_mint)?;

        msg!("Token minted successfully!");
        Ok(())
    }

    pub fn create_stake(
        ctx: Context<CreateStake>,
        reward_mint: Pubkey,
        reward_rate: u64,
    ) -> Result<()> {
        let stake_token_mint: Pubkey = "57dQxpHFknJs96w1Z1DTHi6QgxmR9i7XdhxKuZp8xtzQ"
            .parse()
            .unwrap();
        require!(
            ctx.accounts.staking_mint.key() == stake_token_mint,
            ErrorCode::InvalidStakingMint
        );

        let stake = &mut ctx.accounts.stake;

        stake.admin = ctx.accounts.admin.key();
        stake.staking_mint = ctx.accounts.staking_mint.key();
        stake.reward_mint = reward_mint;
        stake.reward_rate = reward_rate;
        stake.last_update_time = Clock::get()?.unix_timestamp;
        stake.reward_per_token_stored = 0;
        stake.vault_bump=ctx.bumps.vault;
        Ok(())
    }

    pub fn stake_token(
        ctx: Context<StakeToken>,
        amount_to_stake: u64,
        stake_time: i64,
    ) -> Result<()> {
        require!(ctx.accounts.stake.key()==ctx.accounts.user_stake.stake.key(),ErrorCode::InvalidStake);
        require!(ctx.accounts.user_ata.mint==ctx.accounts.stake.staking_mint,ErrorCode::InvalidUserAta);

        let user_stake = &mut ctx.accounts.user_stake;

        user_stake.user = ctx.accounts.user.key();
        user_stake.stake=ctx.accounts.stake.key();
        user_stake.amount_staked = user_stake.amount_staked.saturating_add(amount_to_stake);
        user_stake.pending_reward = 0;
        user_stake.stake_time = stake_time;
        user_stake.start_time = Clock::get()?.unix_timestamp;
        user_stake.stake_debt = 0;
        user_stake.end_time = user_stake.stake_time + user_stake.start_time;
        user_stake.vault_bump=ctx.bumps.vault;

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(
            cpi_program,
            Transfer {
                from: ctx.accounts.user_ata.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );

        transfer(cpi_ctx, amount_to_stake)?;

        emit!(StakePlaced {
            user: ctx.accounts.user.key(),
            stake_amount: amount_to_stake,
            stake_time: stake_time
        });
        Ok(())
    }
}
