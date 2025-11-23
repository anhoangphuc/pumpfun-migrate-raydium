use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::{sync_native, SyncNative};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::Vault;

pub const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(address = NATIVE_MINT)]
    pub wsol_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [Vault::SEED_PREFIX.as_bytes(), mint.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = wsol_mint,
        associated_token::authority = vault
    )]
    pub wsol_vault_token_account: Box<Account<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Swap>) -> Result<()> {
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.wsol_vault_token_account.to_account_info(),
            },
        ),
        20 * LAMPORTS_PER_SOL,
    )?;

    sync_native(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SyncNative {
            account: ctx.accounts.wsol_vault_token_account.to_account_info(),
        },
    ))?;

    Ok(())
}
