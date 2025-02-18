lib.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};

declare_id!("YourProgramIDHere");

#[program]
pub mod aurorarift_nft {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, metadata_uri: String, name: String, symbol: String) -> Result<()> {
        // Mint the NFT
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        // Create metadata for the NFT using Metaplex Token Metadata Program
        invoke(
            &mpl_token_metadata::instruction::create_metadata_accounts_v3(
                mpl_token_metadata::id(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                name,
                symbol,
                metadata_uri,
                None,
                500, // Royalty in basis points (e.g., 500 = 5%)
                true,
                true,
                None,
                None,
            ),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(init, payer = payer, mint::decimals = 0, mint::authority = mint_authority)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(signer)]
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        seeds = [b"metadata", mpl_token_metadata::id().as_ref(), mint.key().as_ref()],
        bump,
        space = 8 + 32 + 64 + 200
    )]
    pub metadata: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,}
