use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

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
    pub underlying_token: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        space = 8 + Mint::LEN,
    )]
    pub asset: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddAssets>, max_supply: u128) -> Result<()> {
    let asset_info = &mut ctx.accounts.asset_info;
    asset_info.initialize(
        ctx.accounts.underlying_token.key(),
        ctx.accounts.asset.key(),
        max_supply,
    );
    Ok(())
}
