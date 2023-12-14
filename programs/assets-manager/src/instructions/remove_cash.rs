use crate::instructions::UpdateCash;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<UpdateCash>, amount: u64) -> Result<()> {
    let asset_info = &mut ctx.accounts.asset_info;
    asset_info.cash -= amount;
    Ok(())
}
