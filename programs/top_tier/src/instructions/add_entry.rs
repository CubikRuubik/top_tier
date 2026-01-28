use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddEntry<'info> {
    #[account(
        mut,
        seeds = [b"leaderboard"],
        bump
    )]
    pub leaderboard: AccountLoader<'info, LeaderBoard>,

    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler_add_entry(
    ctx: Context<AddEntry>,
    hash: [u8; 32],
    metadata: [u8; 128],
) -> Result<()> {
    let mut leaderboard = ctx.accounts.leaderboard.load_mut()?;

    let empty_index = leaderboard
        .entries
        .iter()
        .position(|e| e.score == 0)
        .ok_or(AddEntryError::LeaderboardFull)?;

    leaderboard.entries[empty_index] = Entry {
        hash: hash,
        metadata_uri: metadata,
        score: 1,
    };

    Ok(())
}

#[error_code]
pub enum AddEntryError {
    #[msg("Leaderboard is full")]
    LeaderboardFull,
}
