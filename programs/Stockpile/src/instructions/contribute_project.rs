use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ContributeProject<'info> {
    #[account(init,
    seeds = [
        project.name.as_ref(),
        project.key().as_ref(),
        contributor.key().as_ref(),
        (amount as u64).to_le_bytes().as_ref(),
    ],
    bump,
    payer = contributor,
    space = 8 + 8 + 32 + 32,
     )]
    pub info_acc: Account<'info, Info>,
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    /// CHECK:  This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn contribute_project(ctx: Context<ContributeProject>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let info_acc = &mut ctx.accounts.info_acc;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.contributor.to_account_info(),
                to: ctx.accounts.project.to_account_info(),
            },
        ),
        amount,
    )?;

    info_acc.amount = amount as u64;
    info_acc.contributor = ctx.accounts.contributor.key();
    info_acc.fundraiser = ctx.accounts.project.key();

    ctx.accounts.project.raised += amount as u64;
    ctx.accounts.project.contributions += 1;
    user_account.contributions += 1;

    Ok(())
}
