use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, SyncNative, Token, TokenAccount, sync_native},
};


pub const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = vault,
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(address = NATIVE_MINT)]
    pub wsol_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [Vault::SEED_PREFIX.as_bytes(), mint.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = wsol_mint,
        associated_token::authority = vault
    )]
    pub wsol_vault_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Vault {}

impl Vault {
    pub const SEED_PREFIX: &'static str = "vault";
}

pub fn handler(ctx: Context<CreateToken>) -> Result<()> {
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        ),
        1 * LAMPORTS_PER_SOL,
    )?;

    let mint_key = ctx.accounts.mint.key();
    let vault_seeds: &[&[&[u8]]] = &[&[
        Vault::SEED_PREFIX.as_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps.vault],
    ]];

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            vault_seeds,
        ),
        1000_000_000_000,
    )?;

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.wsol_vault_token_account.to_account_info(),
            },
        ),
        10 * LAMPORTS_PER_SOL,
    )?;

    sync_native(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SyncNative {
            account: ctx.accounts.wsol_vault_token_account.to_account_info(),
        },
    ))?;

    Ok(())
}
