use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::states::*;

#[derive(Accounts)]
pub struct TransferUnderlyingToken<'info> {
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
    #[account(mut)]
    pub underlying_token: Account<'info, Mint>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TransferUnderlyingToken>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.from;
    let to = &mut ctx.accounts.to;
    let token_program = &ctx.accounts.token_program;
    let asset_info = &ctx.accounts.asset_info;

    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
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
