use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct MintAsset<'info> {
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
    pub to: Account<'info, TokenAccount>,
    pub underlying_token: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<MintAsset>, amount: u128) -> Result<()> {
    let to = &mut ctx.accounts.to;
    let token_program = &ctx.accounts.token_program;
    let asset = &ctx.accounts.asset;

    token::mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: asset.to_account_info(),
                to: to.to_account_info(),
                authority: ctx.accounts.asset_info.to_account_info(),
            },
            &[&[
                "asset_info".as_bytes(),
                ctx.accounts.underlying_token.key().as_ref(),
                &[ctx.bumps.asset_info],
            ]],
        ),
        amount as u64,
    )?;
    Ok(())
}
