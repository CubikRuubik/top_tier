use anchor_lang::prelude::*;

#[account]
pub struct Entry {
    pub title: String,
    pub metadata_uri: String,
    pub score: i64,
    pub creator: Pubkey,
}
