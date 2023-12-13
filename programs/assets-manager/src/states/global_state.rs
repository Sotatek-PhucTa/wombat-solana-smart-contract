use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    pub admin: Pubkey,
}

impl GlobalState {
    pub const SPACE: usize = 32;
    pub fn initialize(&mut self, admin: Pubkey) {
        self.admin = admin;
    }
}
