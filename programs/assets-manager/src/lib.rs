mod instructions;
mod states;

use anchor_lang::prelude::*;

use instructions::*;
declare_id!("6pJXuxSNVEV7uX76QWzbhrdZYUhf7XPRDZ7FXVfRKELu");

#[program]
pub mod assets_manager {
    use super::*;

    pub fn initialize(ctx: Context<InitializeState>) -> Result<()> {
        instructions::init_state::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
