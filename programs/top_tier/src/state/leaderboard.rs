use anchor_lang::prelude::*;

#[account(zero_copy)]
#[repr(C)]
pub struct LeaderBoard {
    pub entries: [Entry; 32],
}

#[zero_copy]
#[repr(C)]
pub struct Entry {
    pub hash: [u8; 32],
    pub metadata_uri: [u8; 128],
    pub score: u32,
}
