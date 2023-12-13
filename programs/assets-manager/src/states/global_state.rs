use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    /// Admin address of the program, manage the assets
    pub admin: Pubkey, // 32 bytes
}

impl GlobalState {
    pub const SPACE: usize = 32;
    pub fn initialize(&mut self, admin: Pubkey) {
        self.admin = admin;
    }
}
