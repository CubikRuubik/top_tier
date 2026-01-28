use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("575YdJj5F2sKDWbHaDCfv7NfpDF7c9CTeYd4qu1eYGtp");

#[program]
pub mod top_tier {
    use super::*;

    pub fn initialize(ctx: Context<InitializeLeaderboard>) -> Result<()> {
        instructions::initialize::handler_initialize(ctx)
    }

    pub fn add_entry(ctx: Context<AddEntry>, hash: [u8; 32], metadata: [u8; 128]) -> Result<()> {
        instructions::add_entry::handler_add_entry(ctx, hash, metadata)
    }

    pub fn vote(ctx: Context<Vote>, hash: [u8; 32]) -> Result<()> {
        instructions::vote::handler_vote(ctx, hash)
    }
}
