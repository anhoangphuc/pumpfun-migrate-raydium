use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use raydium_cp_swap::{
    cpi,
    states::{OBSERVATION_SEED, POOL_LP_MINT_SEED, POOL_SEED, POOL_VAULT_SEED},
    AUTH_SEED,
};

use crate::Vault;

pub const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

#[derive(Accounts)]
pub struct MigrateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account()]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(address = NATIVE_MINT)]
    pub wsol_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token_0 mint, the key must smaller then token_1 mint.
    #[account(
        constraint = token_0_mint.key() < token_1_mint.key(),
        mint::token_program = token_0_program
    )]
    pub token_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token_1 mint, the key must grater then token_0 mint.
    #[account(mint::token_program = token_1_program)]
    pub token_1_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: pool lp mint, init by cp-swap
    #[account(
        mut,
        seeds = [
            POOL_LP_MINT_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub lp_mint: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [Vault::SEED_PREFIX.as_bytes(), mint.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = wsol_mint,
        associated_token::authority = vault
    )]
    pub wsol_vault_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK
    #[account(
        mut,
        seeds = [
            vault.key().as_ref(),
            token_0_program.key().as_ref(),
            token_0_mint.key().as_ref(),
        ],
        bump,
        seeds::program = associated_token_program,
    )]
    pub creator_token_0: UncheckedAccount<'info>,

    /// CHECK
    #[account(
        mut,
        seeds = [
            vault.key().as_ref(),
            token_1_program.key().as_ref(),
            token_1_mint.key().as_ref(),
        ],
        bump,
        seeds::program = associated_token_program,
    )]
    pub creator_token_1: UncheckedAccount<'info>,

    /// CHECK
    #[account(
        mut,
        seeds = [
            vault.key().as_ref(),
            token_program.key().as_ref(),
            lp_mint.key().as_ref(),
        ],
        bump,
        seeds::program = associated_token_program,
    )]
    pub creator_lp_token: UncheckedAccount<'info>,

    /// CHECKED: amm config, passed to cp-swap, checked by the raydium initialize instruction
    #[account(
        owner = cp_swap_program.key(),
    )]
    pub amm_config: UncheckedAccount<'info>,

    /// CHECK: create pool fee account, will be checked in the raydium initialize instruction
    #[account(mut)]
    pub create_pool_fee: UncheckedAccount<'info>,

    /// CHECK: lp mint authority, passed to cp-swap
    #[account(
        seeds = [AUTH_SEED.as_bytes()],
        seeds::program = cp_swap_program,
        bump
    )]
    pub authority: UncheckedAccount<'info>,

    /// CHECK: Initialize an account to store the pool state, init by cp-swap
    #[account(
        mut,
        seeds = [
            POOL_SEED.as_bytes(),
            amm_config.key().as_ref(),
            token_0_mint.key().as_ref(),
            token_1_mint.key().as_ref(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub pool_state: UncheckedAccount<'info>,

    /// CHECK: an account to store oracle observations, init by cp-swap
    #[account(
        mut,
        seeds = [
            OBSERVATION_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub observation_state: UncheckedAccount<'info>,

    /// CHECK: Token_0 vault for the pool, init by cp-swap
    #[account(
        mut,
        seeds = [
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_0_mint.key().as_ref()
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub token_0_vault: UncheckedAccount<'info>,

    /// CHECK: Token_1 vault for the pool, init by cp-swap
    #[account(
        mut,
        seeds = [
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_1_mint.key().as_ref()
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub token_1_vault: UncheckedAccount<'info>,

    /// Sysvar for program account
    pub rent: Sysvar<'info, Rent>,

    /// CHECK: Raydium swap program
    #[account(
        address = raydium_cp_swap::program::RaydiumCpSwap::id(),
    )]
    pub cp_swap_program: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    /// Spl token program or token program 2022
    pub token_0_program: Interface<'info, TokenInterface>,
    /// Spl token program or token program 2022
    pub token_1_program: Interface<'info, TokenInterface>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<MigrateToken>) -> Result<()> {
    let mint_key = ctx.accounts.mint.key();
    let vault = ctx.accounts.vault.to_account_info();
    let wsol_vault = ctx.accounts.wsol_vault_token_account.to_account_info();

    let vault_seeds: &[&[&[u8]]] = &[&[
        Vault::SEED_PREFIX.as_bytes(),
        mint_key.as_ref(),
        &[ctx.bumps.vault],
    ]];

    let cpi_accounts = cpi::accounts::Initialize {
        creator: vault,
        amm_config: ctx.accounts.amm_config.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        pool_state: ctx.accounts.pool_state.to_account_info(),
        token_0_mint: ctx.accounts.token_0_mint.to_account_info(),
        token_1_mint: ctx.accounts.token_1_mint.to_account_info(),
        lp_mint: ctx.accounts.lp_mint.to_account_info(),
        creator_token_0: ctx.accounts.creator_token_0.to_account_info(),
        creator_token_1: ctx.accounts.creator_token_1.to_account_info(),
        creator_lp_token: ctx.accounts.creator_lp_token.to_account_info(),
        token_0_vault: ctx.accounts.token_0_vault.to_account_info(),
        token_1_vault: ctx.accounts.token_1_vault.to_account_info(),
        create_pool_fee: ctx.accounts.create_pool_fee.to_account_info(),
        observation_state: ctx.accounts.observation_state.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        token_0_program: ctx.accounts.token_0_program.to_account_info(),
        token_1_program: ctx.accounts.token_1_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.cp_swap_program.to_account_info(),
        cpi_accounts,
        vault_seeds,
    );

    let amount_0: u64;
    let amount_1: u64;
    if ctx.accounts.token_0_mint.key() == NATIVE_MINT {
        amount_0 = 9 * LAMPORTS_PER_SOL;
        amount_1 = 1_000_000_000;
    } else {
        amount_0 = 1_000_000_000;
        amount_1 = 9 * LAMPORTS_PER_SOL;
    }

    cpi::initialize(
        cpi_context,
        amount_0,
        amount_1,
        Clock::get()?.unix_timestamp as u64,
    )?;

    Ok(())
}
