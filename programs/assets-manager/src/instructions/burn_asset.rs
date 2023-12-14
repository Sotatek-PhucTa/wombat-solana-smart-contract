use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct BurnAsset<'info> {
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
        seeds = [b"asset", underlying_token.key().as_ref()],
        bump,
    )]
    pub asset: Account<'info, Mint>,
    #[account(
        seeds = [b"asset_info", underlying_token.key().as_ref()],
        bump,
    )]
    pub asset_info: Account<'info, AssetInfo>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub underlying_token: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<BurnAsset>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.from;
    let token_program = &ctx.accounts.token_program;
    let asset = &ctx.accounts.asset;
    let asset_info = &ctx.accounts.asset_info;

    token::burn(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Burn {
                mint: asset.to_account_info(),
                from: from.to_account_info(),
                authority: asset_info.to_account_info(),
            },
            &[&[
                "asset_info".as_bytes(),
                ctx.accounts.underlying_token.key().as_ref(),
                &[ctx.bumps.asset_info],
            ]],
        ),
        amount,
    )?;
    Ok(())
}
