use anchor_lang::prelude::*;

declare_id!("EcFiVjMzzhp1N1KCS9mrBAWVe2zf9JvFNHMje3VHTYd4");

#[program]
pub mod pumpfun_integrate_raydium {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
