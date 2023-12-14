use anchor_lang::prelude::*;

#[error_code]
pub enum AssetManagerError {
    #[msg("Token reach supply")]
    MaxSupplyReached,
}
