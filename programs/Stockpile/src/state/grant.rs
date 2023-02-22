use anchor_lang::prelude::*;

#[account]
pub struct Grant {
    pub beneficiary: Pubkey,
    pub creator: String,
    pub name: String,
    pub description: String,
    pub image_link: String,
    pub website_link: String,
    pub twitter: String,
    pub discord: String,
    pub telegram: String,
    pub location: String,
    pub repo: String,
    pub goal: String,
    pub raised: u64,
    pub contributions: u8,
    pub is_matching_eligible: bool,
    pub bump: u8,
    pub time: i64,
    pub category: String,
    pub balance: u64,
    pub last_updated: i64,
}

impl Grant {
    pub const LEN: usize = 8 + 8 + 32 + 7000;
}
