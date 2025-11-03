use anchor_lang::prelude::*;

declare_id!("ApsB71rabVYM9UNKLHdXjn8CaSVutWCqmC4MBMzzd9TA");

#[program]
pub mod anvil_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
