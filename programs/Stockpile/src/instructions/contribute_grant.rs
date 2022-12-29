use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ContributeGrant<'info> {
    #[account(init,
    seeds = [
        grant.key().as_ref(),
        contributor.key().as_ref(),
        amount.to_be_bytes().as_ref(),
    ],
    bump,
    payer = contributor,
    space = 8 + 8 + 32 + 32,
     )]
    pub info_acc: Account<'info, Info>,
    #[account(mut)]
    pub grant: Account<'info, Grant>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    /// CHECK:  This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn contribute_grant(ctx: Context<ContributeGrant>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.contributor.to_account_info(),
                to: ctx.accounts.grant.to_account_info(),
            },
        ),
        amount,
    )?;

    ctx.accounts.grant.raised += amount as u64;
    ctx.accounts.grant.contributions += 1;
    user_account.contributions += 1;

    Ok(())
}