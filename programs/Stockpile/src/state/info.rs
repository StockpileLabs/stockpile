use anchor_lang::prelude::*;

#[account]
pub struct Info {
    pub fundraiser: Pubkey,
    pub contributor: Pubkey,
    pub amount: u64,
}

impl Info {
    pub const LEN: usize = 8 + 8 + 8 + 32 + 32;
}
