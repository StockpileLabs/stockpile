use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub authority: Pubkey,
    pub username: String,
    pub fundraisers: u8,
    pub contributions: u8,
    pub bump: u8,
    pub time: i64,
}

impl User {
    pub const LEN: usize = 8 + 8 + 4 + 256;
}
