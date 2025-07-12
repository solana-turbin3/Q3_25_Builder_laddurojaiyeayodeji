#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::{prelude::*, system_program::{ Transfer, transfer}};

declare_id!("6xa9PE6AmBGp56i5jCAHAjQ5bZp1qNPw1PzMiFVhuWug");

#[program]
pub mod anchor_vault_q3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close_account()?;
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
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {

        let rent_exempt = Rent::get()?.minimum_balance(self.vault_state.to_account_info().data_len());

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from : self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let  cpi_ctx  = CpiContext::new(cpi_program, cpi_accounts);
        
        transfer(cpi_ctx, rent_exempt)?;
        
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut
    )]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from : self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let  cpi_ctx  = CpiContext::new(cpi_program, cpi_accounts);
        
        transfer(cpi_ctx, amount)?;
        
        Ok(())
    }
}

#[derive(Accounts)]

pub struct Withdraw<'info> {
    #[account(
        mut
    )]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {

        require!(amount > 0, VaultError::InvalidAmount);

        let vault_balance = self.vault.to_account_info().lamports();
        require!(vault_balance >= amount, VaultError::InsufficientFunds);

        let rent_exempt_minimum = Rent::get()?.minimum_balance(0);
        let remaining_balance = vault_balance
            .checked_sub(amount)
            .ok_or(VaultError::InsufficientFunds)?;
        
        require!(
            remaining_balance >= rent_exempt_minimum,
            VaultError::RentExemptViolation
        );

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from : self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let seeds = &[
            b"vault".as_ref(),
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let  cpi_ctx  = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        
        transfer(cpi_ctx, amount)?;
        
        Ok(())
    }
}


#[derive(Accounts)]

pub struct Close<'info> {
    #[account(
        mut
    )]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> Close<'info> {
    pub fn close_account(&mut self) -> Result<()> {
        
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from : self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let seeds = &[
            b"vault".as_ref(),
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let  cpi_ctx  = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        
        transfer(cpi_ctx, self.vault.lamports())?;
        
        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8
}

// impl Space for VaultState {
//     const INIT_SPACE: usize = 8 + 1 + 1;
// }

#[error_code]
pub enum VaultError {
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Withdrawal would leave vault below rent-exempt minimum")]
    RentExemptViolation,
}