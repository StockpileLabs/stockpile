use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(mut,
            seeds = [
                b"fuckItWeBall!".as_ref(), 
                authority.key().as_ref()], 
                bump = user_account.bump
            )]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update_user(ctx: Context<UpdateUser>, username: String) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    user_account.username = username;

    Ok(())
}
