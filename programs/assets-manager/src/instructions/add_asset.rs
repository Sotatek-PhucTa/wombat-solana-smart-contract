use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct AddAssets<'info> {
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
        init,
        payer = signer,
        space = 8 + AssetInfo::SPACE,
        seeds = [b"asset_info", underlying_token.key().as_ref()],
        bump,
    )]
    pub asset_info: Account<'info, AssetInfo>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = asset_info,
        seeds = [b"asset", underlying_token.key().as_ref()],
        bump,
    )]
    pub asset: Account<'info, Mint>,
    pub underlying_token: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<AddAssets>, max_supply: u64) -> Result<()> {
    let asset_info = &mut ctx.accounts.asset_info;
    asset_info.initialize(
        ctx.accounts.underlying_token.key(),
        ctx.accounts.asset.key(),
        max_supply,
    );
    Ok(())
}
