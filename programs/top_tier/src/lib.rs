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

    pub fn create_entry(
        ctx: Context<CreateEntry>,
        title: String,
        metadata_uri: String,
    ) -> Result<()> {
        instructions::create_entry::handler_create_entry(ctx, title, metadata_uri)
    }

    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        instructions::vote::handler_vote(ctx)
    }
}
