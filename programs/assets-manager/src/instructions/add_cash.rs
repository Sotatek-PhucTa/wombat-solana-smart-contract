use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct UpdateCash<'info> {
    #[account(
        seeds = [b"global_state"],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        address = global_state.admin.key(),
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"asset_info", underlying_token.key().as_ref()],
        bump,
    )]
    pub asset_info: Account<'info, AssetInfo>,
    #[account()]
    pub underlying_token: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<UpdateCash>, amount: u64) -> Result<()> {
    let asset_info = &mut ctx.accounts.asset_info;
    asset_info.cash += amount;
    Ok(())
}
