use anchor_lang::prelude::*;
pub mod instructions;
pub use instructions::*;

declare_id!("J2d5vfhWuhStKkvFi3j7Tk7EUnZfuvZiJ4ZQd4aQqBPy");

#[program]
pub mod stake {
    use anchor_spl::metadata::{create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

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

        let data=DataV2{
            name:token_name,
            symbol:token_symbol,
            uri:token_uri,
            seller_fee_basis_points:0,
            creators:None,
            collection:None,
            uses:None
        };

        create_metadata_accounts_v3(cpi_ctx, data, false, true, None)?;

        msg!("Token created successfully");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
