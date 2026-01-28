use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(entry_hash: [u8; 32])]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [b"leaderboard"],
        bump
    )]
    pub leaderboard: AccountLoader<'info, LeaderBoard>,

    #[account(
        init,
        payer = voter,
        space = 8,
        seeds = [b"vote", voter.key().as_ref(), entry_hash.as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>, // TODO: how can I create a custom error to indicate double vote

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler_vote(ctx: Context<Vote>, entry_hash: [u8; 32]) -> Result<()> {
    let mut leaderboard = ctx.accounts.leaderboard.load_mut()?;

    let index = leaderboard
        .entries
        .iter()
        .position(|e| e.hash == entry_hash)
        .ok_or(VoteError::EntryNotFound)?;

    leaderboard.entries[index].score += 1;

    let mut current_index = index;
    while current_index > 0 {
        let prev_index = current_index - 1;

        if leaderboard.entries[current_index].score > leaderboard.entries[prev_index].score {
            leaderboard.entries.swap(current_index, prev_index);
            current_index = prev_index;
        } else {
            break;
        }
    }

    Ok(())
}

#[error_code]
pub enum VoteError {
    #[msg("Entry not found")]
    EntryNotFound,
}
