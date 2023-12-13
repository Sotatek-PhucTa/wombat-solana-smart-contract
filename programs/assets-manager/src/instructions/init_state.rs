use anchor_lang::prelude::*;

use crate::states::*;
#[derive(Accounts)]
pub struct InitializeState<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"global_state"],
        bump,
        space = 8 + GlobalState::SPACE,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeState>) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    global_state.initialize(ctx.accounts.signer.key());
    Ok(())
}
