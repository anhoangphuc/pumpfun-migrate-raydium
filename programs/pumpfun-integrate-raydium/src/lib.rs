use anchor_lang::prelude::*;

pub mod instructions;

pub use instructions::*;

declare_id!("EcFiVjMzzhp1N1KCS9mrBAWVe2zf9JvFNHMje3VHTYd4");

#[program]
pub mod pumpfun_integrate_raydium {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_token(ctx: Context<CreateToken>) -> Result<()> {
        instructions::create_token::handler(ctx)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>) -> Result<()> {
        instructions::swap::handler(ctx)?;
        Ok(())
    }

    pub fn migrate_token(ctx: Context<MigrateToken>) -> Result<()> {
        instructions::migrate_token::handler(ctx)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
