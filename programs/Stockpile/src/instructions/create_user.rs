use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init,
            seeds = [b"fuckItWeBall!".as_ref(),
            authority.key().as_ref()],
            bump,
            payer = authority,
            space = User::LEN,
        )]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let authority = &mut ctx.accounts.authority;

    user_account.username = username;
    user_account.bump = *ctx.bumps.get("user_account").unwrap();
    user_account.authority = authority.key();
    user_account.time = Clock::get()?.unix_timestamp;

    Ok(())
}
