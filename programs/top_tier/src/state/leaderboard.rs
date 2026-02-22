use anchor_lang::prelude::*;

#[zero_copy]
#[repr(C)]
pub struct LeaderboardEntry {
    pub pubkey: Pubkey,
    pub score: u64,
}

#[account(zero_copy)]
#[repr(C)]
pub struct LeaderBoard {
    pub entries: [LeaderboardEntry; 32],
    pub count: u64,
}
