use anchor_lang::prelude::*;

declare_id!("C35VTXEEBrWAcQ3tCHh7x45ZRWLpu9KSQpTxfPHR8CYo");

#[program]
pub mod wombat_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
