mod error;
mod instructions;
mod states;

use anchor_lang::prelude::*;

use instructions::*;
declare_id!("6pJXuxSNVEV7uX76QWzbhrdZYUhf7XPRDZ7FXVfRKELu");

#[program]
pub mod assets_manager {
    use super::*;

    pub fn initialize(ctx: Context<InitializeState>) -> Result<()> {
        init_state::handler(ctx)
    }

    pub fn add_asset(ctx: Context<AddAssets>, max_supply: u64) -> Result<()> {
        add_asset::handler(ctx, max_supply)
    }

    pub fn set_max_supply(ctx: Context<SetMaxSupply>, max_supply: u64) -> Result<()> {
        set_max_supply::handler(ctx, max_supply)
    }

    pub fn transfer_underlying_token(
        ctx: Context<TransferUnderlyingToken>,
        amount: u64,
    ) -> Result<()> {
        transfer_underlying_token::handler(ctx, amount)
    }

    pub fn mint(ctx: Context<MintAsset>, amount: u64) -> Result<()> {
        mint_asset::handler(ctx, amount)
    }

    pub fn add_cash(ctx: Context<AddCash>, amount: u64) -> Result<()> {
        add_cash::handler(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
