use anchor_lang::prelude::*;
use solana_program::hash::hash;

declare_id!("11111111111111111111111111111111");

pub const MAX_TITLE_LENGTH: usize = 64;
pub const MAX_METADATA_LENGTH: usize = 256;
pub const LEADERBOARD_SIZE: usize = 100;

#[program]
pub mod tierlist {
    use super::*;

    pub fn initialize_leaderboard(ctx: Context<InitializeLeaderboard>) -> Result<()> {
        let lb = &mut ctx.accounts.leaderboard;
        lb.count = 0;
        Ok(())
    }

    pub fn create_entry(ctx: Context<CreateEntry>, title: String, metadata: String) -> Result<()> {
        require!(title.len() <= MAX_TITLE_LENGTH, TierlistError::TitleTooLong);
        require!(
            metadata.len() <= MAX_METADATA_LENGTH,
            TierlistError::MetadataTooLong
        );

        let entry = &mut ctx.accounts.entry;
        entry.authority = ctx.accounts.creator.key();
        entry.title = title;
        entry.metadata = metadata;
        entry.score = 1;

        update_leaderboard(&mut ctx.accounts.leaderboard, entry.key(), entry.score);

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        entry.score = entry.score.checked_add(1).ok_or(TierlistError::Overflow)?;

        update_leaderboard(&mut ctx.accounts.leaderboard, entry.key(), entry.score);

        Ok(())
    }
}

/* ───────────────────────── Leaderboard logic ───────────────────────── */

fn update_leaderboard(leaderboard: &mut Leaderboard, entry_pubkey: Pubkey, entry_score: u64) {
    let mut len = leaderboard.count as usize;

    /* 1️⃣ Remove existing entry if present */
    if let Some(pos) = leaderboard.entries[..len]
        .iter()
        .position(|e| e.entry == entry_pubkey)
    {
        for i in pos..len - 1 {
            leaderboard.entries[i] = leaderboard.entries[i + 1];
        }
        len -= 1;
        leaderboard.count = len as u8;
    }

    /* 2️⃣ Find insertion index */
    let mut insert_pos = len;
    for i in 0..len {
        if entry_score > leaderboard.entries[i].score {
            insert_pos = i;
            break;
        }
    }

    /* 3️⃣ If full and doesn't qualify → exit */
    if len == LEADERBOARD_SIZE && insert_pos == len {
        return;
    }

    /* 4️⃣ Shift down */
    let new_len = LEADERBOARD_SIZE.min(len + 1);
    for i in (insert_pos..new_len - 1).rev() {
        leaderboard.entries[i + 1] = leaderboard.entries[i];
    }

    /* 5️⃣ Insert */
    leaderboard.entries[insert_pos] = LeaderboardEntry {
        entry: entry_pubkey,
        score: entry_score,
    };

    leaderboard.count = new_len as u8;
}

/* ───────────────────────── Accounts ───────────────────────── */

#[derive(Accounts)]
pub struct InitializeLeaderboard<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Leaderboard::INIT_SPACE,
        seeds = [b"leaderboard"],
        bump
    )]
    pub leaderboard: Account<'info, Leaderboard>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Entry::INIT_SPACE,
        seeds = [b"entry", hash(title.as_bytes()).to_bytes().as_ref()],
        bump
    )]
    pub entry: Account<'info, Entry>,

    #[account(
        init,
        payer = creator,
        space = 8 + VoteRecord::INIT_SPACE,
        seeds = [b"vote", entry.key().as_ref(), creator.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,

    #[account(
        mut,
        seeds = [b"leaderboard"],
        bump
    )]
    pub leaderboard: Account<'info, Leaderboard>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub entry: Account<'info, Entry>,

    #[account(
        init,
        payer = voter,
        space = 8 + VoteRecord::INIT_SPACE,
        seeds = [b"vote", entry.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,

    #[account(
        mut,
        seeds = [b"leaderboard"],
        bump
    )]
    pub leaderboard: Account<'info, Leaderboard>,

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/* ───────────────────────── State ───────────────────────── */

#[account]
#[derive(InitSpace)]
pub struct Entry {
    pub authority: Pubkey,
    #[max_len(MAX_TITLE_LENGTH)]
    pub title: String,
    #[max_len(MAX_METADATA_LENGTH)]
    pub metadata: String,
    pub score: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct LeaderboardEntry {
    pub entry: Pubkey,
    pub score: u64,
}

impl Space for LeaderboardEntry {
    const INIT_SPACE: usize = 32 + 8;
}

#[account]
#[derive(InitSpace)]
pub struct Leaderboard {
    pub entries: [LeaderboardEntry; LEADERBOARD_SIZE],
    pub count: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VoteRecord {}

/* ───────────────────────── Errors ───────────────────────── */

#[error_code]
pub enum TierlistError {
    #[msg("Title exceeds maximum length")]
    TitleTooLong,
    #[msg("Metadata exceeds maximum length")]
    MetadataTooLong,
    #[msg("Score overflow")]
    Overflow,
}
