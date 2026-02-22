use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [b"leaderboard"],
        bump
    )]
    pub leaderboard: AccountLoader<'info, LeaderBoard>,

    #[account(mut)]
    pub entry: Account<'info, Entry>,

    #[account(
        init,
        payer = voter,
        space = 8,
        seeds = [b"vote", voter.key().as_ref(), entry.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>, // TODO: how can I create a custom error to indicate double vote

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler_vote(ctx: Context<Vote>) -> Result<()> {
    let entry = &mut ctx.accounts.entry;
    entry.score += 1;

    let entry_key = entry.key();
    let entry_score = entry.score;
    let mut leaderboard = ctx.accounts.leaderboard.load_mut()?;
    let count = leaderboard.count as usize;

    let position = leaderboard.entries[..count]
        .iter()
        .position(|e| e.pubkey == entry_key);

    match position {
        Some(index) => {
            leaderboard.entries[index].score = entry_score;

            let mut current = index;
            while current > 0 {
                let prev = current - 1;
                if leaderboard.entries[current].score > leaderboard.entries[prev].score {
                    leaderboard.entries.swap(current, prev);
                    current = prev;
                } else {
                    break;
                }
            }
        }
        None => {
            if count < 32 {
                leaderboard.entries[count] = LeaderboardEntry {
                    pubkey: entry_key,
                    score: entry_score,
                };
                leaderboard.count += 1;

                bubble_up(&mut leaderboard.entries, count);
            } else {
                let last_score = leaderboard.entries[31].score;
                if entry_score > last_score {
                    leaderboard.entries[31] = LeaderboardEntry {
                        pubkey: entry_key,
                        score: entry_score,
                    };

                    bubble_up(&mut leaderboard.entries, 31);
                }
            }
        }
    }

    Ok(())
}

fn bubble_up(entries: &mut [LeaderboardEntry], start_index: usize) {
    let mut current = start_index;
    while current > 0 {
        let prev = current - 1;
        if entries[current].score > entries[prev].score {
            entries.swap(current, prev);
            current = prev;
        } else {
            break;
        }
    }
}

#[error_code]
pub enum VoteError {
    #[msg("Entry not found")]
    EntryNotFound,
}
