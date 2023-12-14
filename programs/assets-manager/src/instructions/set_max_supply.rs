use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::states::*;

#[derive(Accounts)]
pub struct SetMaxSupply<'info> {
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
    pub underlying_token: Account<'info, Mint>,
}

pub fn handler(ctx: Context<SetMaxSupply>, max_supply: u64) -> Result<()> {
    let asset_info = &mut ctx.accounts.asset_info;
    asset_info.set_max_supply(max_supply);
    Ok(())
}
