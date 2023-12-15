use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct UpdateLiability<'info> {
    #[account(
        mut,
        address = asset_info.pool.key(),
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

pub fn handler(ctx: Context<UpdateLiability>, amount: u64) -> Result<()> {
    let asset_info = &mut ctx.accounts.asset_info;
    asset_info.add_liability(amount);
    Ok(())
}
