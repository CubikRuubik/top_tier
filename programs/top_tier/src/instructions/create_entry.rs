use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 4 + 32 + 4 + 128 + 8 + 32,
        seeds = [b"entry", title.as_bytes()],
        bump
    )]
    pub entry: Account<'info, Entry>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler_create_entry(
    ctx: Context<CreateEntry>,
    title: String,
    metadata_uri: String,
) -> Result<()> {
    let entry = &mut ctx.accounts.entry;
    entry.title = title;
    entry.metadata_uri = metadata_uri;
    entry.score = 0;
    entry.creator = ctx.accounts.creator.key();
    Ok(())
}
