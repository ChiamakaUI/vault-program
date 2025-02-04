use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("Gdmd9nosPEQ9sozQSscEv3wBUR89EdLxZFCXsNJsHpNv");

#[program]
pub mod vault {
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
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough funds in the vault.")]
    InsufficientFunds,
}


#[derive(Accounts)]

pub struct Initialize<'info> {
    #[account(init, 
        payer=owner,
        space=8 + VaultState::INIT_SPACE, 
        seeds=[b"state", owner.key().as_ref()], 
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(seeds=[vault_state.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, 
        seeds=[b"state", 
        signer.key().as_ref()], 
        bump=vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(seeds=[vault_state.key().as_ref()], bump=vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount:u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(system_program, accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]

pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, 
        seeds=[b"state", 
        signer.key().as_ref()], 
        bump=vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut, seeds=[vault_state.key().as_ref()], bump=vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();

        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let vault_key = self.vault_state.key();
        let vault_bump = self.vault_state.vault_bump;

        let vault_seeds = [vault_key.as_ref(), &[vault_bump]];

        let signer_seeds: &[&[&[u8]]] = &[&vault_seeds];

        let cpi_ctx = CpiContext::new_with_signer(system_program, accounts, signer_seeds);

        // assert!(self.vault.lamports() >= amount);
        // Check if there is balance in the vault and throw an error if there isn't
        if self.vault.lamports() < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]

pub struct CloseVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>
}

impl<'info> CloseVault<'info> {
    pub fn close_vault(&mut self) -> Result<()>{

        Ok(())
    }
}