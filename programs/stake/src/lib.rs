use anchor_lang::prelude::*;
pub mod error;
pub mod event;
pub mod instructions;
pub mod states;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
    },
    token::{mint_to, MintTo,transfer, Transfer},
};
pub use error::ErrorCode;
pub use event::*;
pub use instructions::*;
pub use states::*;

declare_id!("3Yotwu7h86XRhWEbVnbAUJQGxmBEypEwN5ZoLQBXrT8G");

#[program]
pub mod stake {
    use super::*;

    const REWARD_PRECISION: u128 = 1_000_000_000_000;

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

        msg!("Creating stake account for admin: {}", ctx.accounts.admin.key());

        stake.admin = ctx.accounts.admin.key();
        stake.staking_mint = ctx.accounts.staking_mint.key();
        stake.reward_mint = stake_token_mint;
        stake.reward_rate = reward_rate;
        stake.last_update_time = Clock::get()?.unix_timestamp;
        stake.reward_per_token_stored = 0;
        stake.vault_bump = ctx.bumps.vault;

        msg!("Stake account initialized: ");

        Ok(())
    }

    pub fn stake_token(
        ctx: Context<StakeToken>,
        amount_to_stake: u64,
        stake_time: i64,
    ) -> Result<()> {
        // require!(
        //     ctx.accounts.stake.key() == ctx.accounts.user_stake.stake.key(),
        //     ErrorCode::InvalidStake
        // );
        // require!(
        //     ctx.accounts.user_ata.mint == ctx.accounts.stake.staking_mint,
        //     ErrorCode::InvalidUserAta
        // );

        let user_stake = &mut ctx.accounts.user_stake;
        let stake = &mut ctx.accounts.stake;

        let curent_time = Clock::get()?.unix_timestamp;
        let last_update_time = stake.last_update_time;
        let time_elapsed = curent_time - last_update_time;
        let total_staked = stake.total_staked;
        let reward_rate = stake.reward_rate;
        let mut reward_per_token_stored = stake.reward_per_token_stored;

        if total_staked > 0 {
            let additional_reward = (time_elapsed as u128)
                .checked_mul(reward_rate as u128)
                .unwrap()
                .checked_div(total_staked as u128)
                .unwrap();

            reward_per_token_stored = reward_per_token_stored
                .checked_add(additional_reward)
                .unwrap();
        }

        stake.reward_per_token_stored=reward_per_token_stored;
        stake.last_update_time = curent_time;
        stake.total_staked=stake.total_staked.saturating_add(amount_to_stake);
        

        user_stake.user = ctx.accounts.user.key();
        user_stake.stake = ctx.accounts.stake.key();
        user_stake.amount_staked = user_stake.amount_staked.saturating_add(amount_to_stake);
        user_stake.pending_reward = 0;
        user_stake.stake_time = stake_time;
        user_stake.start_time = curent_time;
        user_stake.stake_debt = reward_per_token_stored;
        user_stake.end_time = user_stake.stake_time + user_stake.start_time;
        user_stake.vault_bump = ctx.bumps.vault;

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

    pub fn unstake_token(ctx: Context<UnstakeToken>) -> Result<()> {
        let stake = &mut ctx.accounts.stake;
        let user_stake = &mut ctx.accounts.user_stake;
        require!(
            user_stake.end_time <= Clock::get()?.unix_timestamp,
            ErrorCode::StakeTimeNotCompleted
        );

        let curent_time = Clock::get()?.unix_timestamp;
        let time_elapsed = curent_time - stake.last_update_time;

        if stake.total_staked > 0 {
            let additional_reward = (time_elapsed as u128)
                .checked_mul(stake.reward_rate as u128)
                .unwrap()
                .checked_mul(REWARD_PRECISION)
                .unwrap()
                .checked_div(stake.total_staked as u128)
                .unwrap();

            stake.reward_per_token_stored = stake
                .reward_per_token_stored
                .checked_add(additional_reward)
                .unwrap();
        }
        stake.last_update_time = curent_time;

        stake.total_staked = stake.total_staked.saturating_sub(user_stake.amount_staked);
        // let reward_debt=user_stake.amount_staked as u128*stake.reward_per_token_stored;

        let pending_reward = user_stake.amount_staked as u128
            * (stake.reward_per_token_stored - user_stake.stake_debt as u128)
            / REWARD_PRECISION;

            user_stake.pending_reward = pending_reward as u64;
            user_stake.stake_debt = stake.reward_per_token_stored;
            
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let vault_key = ctx.accounts.vault.clone().key();
            let signer_seeds: &[&[&[u8]]] = &[&[b"vault_authority", vault_key.as_ref(), &[ctx.bumps.vault_authority]]];
            
            let cpi_ctx = CpiContext::new_with_signer(
                cpi_program,
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.user_ata.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
                signer_seeds,
            );
            
            transfer(cpi_ctx, user_stake.amount_staked)?;
            user_stake.amount_staked = 0;
        Ok(())
    }

    pub fn claim_reward(ctx:Context<ClaimReward>)->Result<()>{
        let user_stake=&mut ctx.accounts.user_stake;
        let stake=&mut ctx.accounts.stake;

        let curent_time = Clock::get()?.unix_timestamp;
        let time_elapsed = curent_time - stake.last_update_time;

        if stake.total_staked > 0 {
            let additional_reward = (time_elapsed as u128)
                .checked_mul(stake.reward_rate as u128)
                .unwrap()
                .checked_mul(REWARD_PRECISION)
                .unwrap()
                .checked_div(stake.total_staked as u128)
                .unwrap();

            stake.reward_per_token_stored = stake
                .reward_per_token_stored
                .checked_add(additional_reward)
                .unwrap();
        }
        stake.last_update_time = curent_time;

         let pending_reward = user_stake.amount_staked as u128
            * (stake.reward_per_token_stored - user_stake.stake_debt as u128)
            / REWARD_PRECISION;

        let cpi_program=ctx.accounts.token_program.to_account_info();
        let vault_key=ctx.accounts.vault.key();
        let signer_seeds: &[&[&[u8]]]=&[&[b"vault_authority",vault_key.as_ref(),&[ctx.bumps.vault_authority]]];

        let cpi_ctx=CpiContext::new_with_signer(cpi_program, Transfer{
            from:ctx.accounts.vault.to_account_info(),
            to:ctx.accounts.user_ata.to_account_info(),
            authority:ctx.accounts.vault_authority.to_account_info()
        }, signer_seeds);

        transfer(cpi_ctx, pending_reward as u64)?;
        user_stake.stake_debt=stake.reward_per_token_stored;
        user_stake.pending_reward = 0;

        Ok(())
    }
}
