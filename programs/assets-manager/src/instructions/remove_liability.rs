use crate::instructions::add_liability::UpdateLiability;
use anchor_lang::context::Context;

pub fn handler(ctx: Context<UpdateLiability>, amount: u64) -> anchor_lang::Result<()> {
    let asset_info = &mut ctx.accounts.asset_info;
    asset_info.liability -= amount;
    Ok(())
}
