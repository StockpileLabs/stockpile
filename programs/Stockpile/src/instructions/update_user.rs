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

pub fn update_user(ctx: Context<UpdateUser>, username: String, image: String) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    if username.len() > 0 {
        user_account.username = username;
    }

    if image.len() > 0 {
        user_account.image = image;
    }

    Ok(())
}
