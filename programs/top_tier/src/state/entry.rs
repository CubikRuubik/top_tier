use anchor_lang::prelude::*;

#[account]
pub struct Entry {
    pub title: String,
    pub metadata_uri: String,
    pub score: u64,
    pub creator: Pubkey,
}
