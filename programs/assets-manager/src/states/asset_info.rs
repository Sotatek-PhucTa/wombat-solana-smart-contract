use anchor_lang::prelude::*;

#[account]
pub struct AssetInfo {
    /// The underlying token that assets peg into
    pub underlying_token: Pubkey, //32 bytes
    /// The asset address, a mint pda
    pub asset: Pubkey, // 32 bytes
    /// The cash value for the assets
    pub cash: u128, // 16 bytes
    /// The liability value for the assets
    pub liability: u128, // 16 bytes
    /// The max supply for the assets
    pub max_supply: u128, // 16 bytes
}

impl AssetInfo {
    pub const SPACE: usize = 32 * 2 + 16 * 3;

    pub fn initialize(
        &mut self,
        underlying_token: Pubkey,
        asset_address: Pubkey,
        max_supply: u128,
    ) {
        self.underlying_token = underlying_token;
        self.asset = asset_address;
        self.cash = 0;
        self.liability = 0;
        self.max_supply = max_supply;
    }
}
