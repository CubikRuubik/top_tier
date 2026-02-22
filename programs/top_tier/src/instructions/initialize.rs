use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeLeaderboard<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<LeaderBoard>(),
        seeds = [b"leaderboard"],
        bump
    )]
    pub leaderboard: AccountLoader<'info, LeaderBoard>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler_initialize(_ctx: Context<InitializeLeaderboard>) -> Result<()> {
    Ok(())
}
