use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};


declare_id!("ApsB71rabVYM9UNKLHdXjn8CaSVutWCqmC4MBMzzd9TA");

#[program]
pub mod anvil_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }
    
    pub fn deposit(ctx:Context<VaultContext>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }
    
    pub fn withdraw(ctx: Context<VaultContext>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
    
    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        ctx.accounts.close_vault()?;
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"vault state", user.key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> Initialize<'info> {
    pub fn initialize (&mut self, bumps: &InitializeBumps) -> Result<()>{ 
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.vault_state_bump = bumps.vault_state; 
        
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
   pub vault_bump: u8,
   pub vault_state_bump: u8,
}


#[derive(Accounts)]
pub struct VaultContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"vault state", user.key().as_ref()],
        bump = vault_state.vault_state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl <'info> VaultContext<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        
        let cpi_accounts = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
        
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        
        transfer(cpi_context, amount)?;
        
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        }; 
        let seeds = &[
            b"vault",
            self.user.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        
        let signer_seeds = &[&seeds[..]];
        
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        
        transfer(cpi_context, amount)?;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct CloseVault <'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        close = user,
        seeds = [b"vault state", user.key().as_ref()],
        bump = vault_state.vault_state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> CloseVault <'info> {
    pub fn close_vault(&mut self) -> Result<()> {
        let balance = self.vault.to_account_info().lamports();
       
       let cpi_program = self.system_program.to_account_info();
      let cpi_accounts = Transfer{
          from: self.vault.to_account_info(),
          to: self.user.to_account_info(),
      };
      let seeds = &[
          b"vault",
          self.user.to_account_info().key.as_ref(),
          &[self.vault_state.vault_bump],
      ];
      
      let signed_seeds = &[&seeds[..]];
      
      let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signed_seeds);
      
      transfer(cpi_context, balance)?;
        Ok(())
    }
}