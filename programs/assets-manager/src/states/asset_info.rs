use anchor_lang::prelude::*;

#[account]
pub struct AssetInfo {
    /// The underlying token that assets peg into
    pub underlying_token: Pubkey, //32 bytes
    /// The asset address, a mint pda
    pub asset: Pubkey, // 32 bytes
    /// The pool address of asset, used to invoke function on asset
    pub pool: Pubkey, // 32 bytes
    /// The cash value for the assets
    pub cash: u64, // 8 bytes
    /// The liability value for the assets
    pub liability: u64, // 8 bytes
    /// The max supply for the assets
    pub max_supply: u64, // 8 bytes
}

impl AssetInfo {
    pub const SPACE: usize = 32 * 3 + 8 * 3;

    pub fn initialize(
        &mut self,
        underlying_token: Pubkey,
        asset_address: Pubkey,
        pool: Pubkey,
        max_supply: u64,
    ) {
        self.underlying_token = underlying_token;
        self.asset = asset_address;
        self.pool = pool;
        self.cash = 0;
        self.liability = 0;
        self.max_supply = max_supply;
    }

    pub fn set_max_supply(&mut self, max_supply: u64) {
        self.max_supply = max_supply;
    }

    pub fn add_cash(&mut self, amount: u64) {
        self.cash += amount;
    }

    pub fn remove_cash(&mut self, amount: u64) {
        self.cash -= amount;
    }
}
