use anchor_lang::prelude::*;

declare_id!("6pJXuxSNVEV7uX76QWzbhrdZYUhf7XPRDZ7FXVfRKELu");

#[program]
pub mod assets_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
