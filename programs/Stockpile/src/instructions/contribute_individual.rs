use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ContributeIndividual<'info> {
    #[account(mut)]
    pub individual: Account<'info, Individual>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    /// CHECK:  This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn contribute_individual(ctx: Context<ContributeIndividual>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.contributor.to_account_info(),
                to: ctx.accounts.individual.to_account_info(),
            },
        ),
        amount,
    )?;

    ctx.accounts.individual.raised += amount as u64;
    ctx.accounts.individual.contributions += 1;
    ctx.accounts.individual.balance += amount as u64;
    user_account.contributions += 1;

    Ok(())
}
